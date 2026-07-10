#!/usr/bin/env python3
# NO-EXISTING-TOOL: this is meos-sys's catalog-driven FFI generator; no prior
# generator exists (the FFI is otherwise produced by rust-bindgen).
# BINDING-HEADER-PARSE-OK: reads the MEOS-API catalog `meos-idl.json`, never a
# `.h` header — the `read_text` call loads that JSON.
"""Generate the ``meos-sys`` FFI from the MEOS-API catalog (``meos-idl.json``).

``meos-sys`` is a low-level, raw FFI to libmeos. Its bindings are a pure
projection of the MEOS-API catalog — the one place a MEOS header is ever parsed
(MEOS-API ``run.py``, libclang). This generator consumes the catalog's
``structs`` / ``functions`` / ``enums`` / ``macros`` arrays and emits a single
``src/generated.rs``:

  * ``pub type`` aliases for the MEOS base-type vocabulary (``Datum``,
    ``TimestampTz``, ``uint8`` …) — a fixed boundary table, not a heuristic;
  * ``#[repr(C)]`` structs from ``structs`` (offset-ordered fields), opaque
    forward declarations for the ones the public headers leave incomplete;
  * ``pub type <E> = c_uint`` + ``pub const <E>_<V>`` for each ``enum``
    (matching rust-bindgen's constified-enum naming, which the upper ``meos``
    crate consumes as ``interpType_DISCRETE`` etc.);
  * ``pub const`` for each public object-like integer ``#define`` (``WKB_NDR`` …);
  * one ``extern "C"`` block per function.

Run:  python3 codegen.py <meos-idl.json> <out.rs>
"""
import json
import re
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Boundary table (keyed on the catalog's declared C type spelling).
#
# NAMED base typedefs are emitted as `pub type` aliases so the names the FFI
# exposes match what rust-bindgen produced (e.g. `meos_sys::TimestampTz`).
# Order is irrelevant — Rust resolves type aliases regardless of definition
# order.
# ---------------------------------------------------------------------------
C_SCHAR = "::std::os::raw::c_schar"
C_SHORT = "::std::os::raw::c_short"
C_INT = "::std::os::raw::c_int"
C_LONG = "::std::os::raw::c_long"
C_UCHAR = "::std::os::raw::c_uchar"
C_USHORT = "::std::os::raw::c_ushort"
C_UINT = "::std::os::raw::c_uint"
C_ULONG = "::std::os::raw::c_ulong"
C_CHAR = "::std::os::raw::c_char"
C_VOID = "::std::os::raw::c_void"

NAMED_ALIASES = [
    ("int8", C_SCHAR),
    ("int16", C_SHORT),
    ("int32", C_INT),
    ("int64", C_LONG),
    ("uint8", C_UCHAR),
    ("uint16", C_USHORT),
    ("uint32", C_UINT),
    ("uint64", C_ULONG),
    ("Datum", "usize"),
    ("size_t", "usize"),
    ("Oid", C_UINT),
    ("lwflags_t", "u16"),
    ("DateADT", "int32"),
    ("TimeADT", "int64"),
    ("Timestamp", "int64"),
    ("TimestampTz", "int64"),
    ("TimeOffset", "int64"),
]
NAMED_ALIAS_SET = {n for n, _ in NAMED_ALIASES}

# Callback typedefs — emitted as typed `Option<unsafe extern "C" fn(…)>` so a
# caller passes a concrete Rust `fn` (`Some(handler)`), exactly as rust-bindgen
# rendered them. A MEOS public callback typedef carries no field layout in the
# catalog, so its (stable) signature lives here in the boundary table.
CALLBACK_ALIASES = {
    "error_handler_fn":
        "::std::option::Option<unsafe extern \"C\" fn("
        f"arg1: {C_INT}, arg2: {C_INT}, arg3: *const {C_CHAR})>",
}

# Primitive C spellings inlined directly to their Rust c-type (no named alias
# needed — nothing references these by name).
INLINE_SCALARS = {
    "bool": "bool",
    "char": C_CHAR,
    "int": C_INT,
    "short": C_SHORT,
    "long": C_LONG,
    "unsigned int": C_UINT,
    "unsigned char": C_UCHAR,
    "unsigned short": C_USHORT,
    "unsigned long": C_ULONG,
    "double": "f64",
    "float": "f32",
    "float4": "f32",
    "float8": "f64",
    "int8_t": C_SCHAR,
    "int16_t": C_SHORT,
    "int32_t": C_INT,
    "int64_t": C_LONG,
    "uint8_t": C_UCHAR,
    "uint16_t": C_USHORT,
    "uint32_t": C_UINT,
    "uint64_t": C_ULONG,
}

# Bare typedef names that are really (function-)pointers. A pointer is ABI
# identical to `*mut c_void`, which is how a raw FFI surfaces an opaque
# callback / context handle (the rare caller casts the concrete value in).
FORCE_VOID_PTR = {
    "datum_func2",
    "meos_malloc_fn",
    "meos_realloc_fn",
    "meos_free_fn",
    "GEOSContextHandle_t",
    # `Numeric` is `NumericData *` — a pointer typedef used bare.
    "Numeric",
}

# Rust keywords + primitives that cannot be used bare as an identifier; a
# colliding field / parameter / function name is suffixed with `_` (matching
# rust-bindgen: `str` -> `str_`, `type` -> `type_`).
RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    "try", "str",
}


def sanitize_ident(name: str, index: int) -> str:
    if not name:
        return f"arg{index + 1}"
    return f"{name}_" if name in RUST_KEYWORDS else name


class Emitter:
    def __init__(self, idl: dict):
        self.struct_names = {s["name"] for s in idl["structs"]}
        self.enum_names = {e["name"] for e in idl["enums"]}
        self.structs = idl["structs"]
        self.enums = idl["enums"]
        self.macros = idl.get("macros", [])
        self.functions = idl["functions"]
        # External opaque types referenced but not declared by the catalog
        # (Jsonb, json_object, gsl_rng, …) — collected while mapping, emitted as
        # zero-sized forward declarations.
        self.opaque: set[str] = set()

    # -- C type -> Rust FFI type -------------------------------------------
    def base_rust(self, name: str) -> str:
        name = name.strip()
        if name == "void":
            return C_VOID
        if name in INLINE_SCALARS:
            return INLINE_SCALARS[name]
        if name in NAMED_ALIAS_SET or name in CALLBACK_ALIASES \
                or name in self.struct_names or name in self.enum_names:
            return name
        # Unknown named type -> opaque forward declaration.
        if re.match(r"^[A-Za-z_]\w*$", name):
            self.opaque.add(name)
            return name
        # Anything exotic left (e.g. a residual expression) -> void.
        return C_VOID

    def rust_type(self, ctype: str) -> str:
        s = ctype.strip()

        # Array: `T[N]` (fixed) or `T[]` (flexible member -> zero-length array).
        m = re.match(r"^(.*?)\s*\[(\d*)\]$", s)
        if m:
            inner = self.rust_type(m.group(1).strip())
            n = m.group(2)
            return f"[{inner}; 0]" if n == "" else f"[{inner}; {n}usize]"

        # Function pointer (inline `T (*)(…)`) -> opaque pointer.
        if "(" in s:
            return f"*mut {C_VOID}"

        stars = s.count("*")
        without_stars = s.replace("*", "")
        base_name = re.sub(r"\b(const|struct|union|enum)\b", " ", without_stars)
        base_name = " ".join(base_name.split())

        if stars == 0:
            if base_name in FORCE_VOID_PTR:
                return f"*mut {C_VOID}"
            return self.base_rust(base_name)

        pointee_const = bool(re.match(r"^\s*const\b", without_stars.strip()))
        inner = self.base_rust(base_name)
        rt = f"*{'const' if pointee_const else 'mut'} {inner}"
        for _ in range(stars - 1):  # outer pointers are non-const here
            rt = f"*mut {rt}"
        return rt

    # -- emitters -----------------------------------------------------------
    def emit_aliases(self) -> str:
        lines = [f"pub type {n} = {t};" for n, t in NAMED_ALIASES]
        lines += [f"pub type {n} = {t};" for n, t in CALLBACK_ALIASES.items()]
        return "\n".join(lines)

    def emit_opaque(self) -> str:
        # Emitted after mapping so `self.opaque` is fully populated.
        lines = []
        for name in sorted(self.opaque):
            lines.append("#[repr(C)]")
            lines.append("#[derive(Debug, Copy, Clone)]")
            lines.append(f"pub struct {name} {{")
            lines.append("    _unused: [u8; 0],")
            lines.append("}")
        return "\n".join(lines)

    def emit_enums(self) -> str:
        out = []
        for e in self.enums:
            name = e["name"]
            values = e["values"]
            repr_ty = C_INT if any(v["value"] < 0 for v in values) else C_UINT
            for v in values:
                out.append(f"pub const {name}_{v['name']}: {name} = {v['value']};")
            out.append(f"pub type {name} = {repr_ty};")
        return "\n".join(out)

    def emit_macros(self) -> str:
        out = []
        for mac in sorted(self.macros, key=lambda x: x["name"]):
            value = mac["value"]
            ty = "i32" if value < 0 else "u32"
            out.append(f"pub const {mac['name']}: {ty} = {value};")
        return "\n".join(out)

    def emit_structs(self) -> str:
        out = []
        for s in self.structs:
            out.append("#[repr(C)]")
            out.append("#[derive(Debug, Copy, Clone)]")
            out.append(f"pub struct {s['name']} {{")
            if not s["fields"]:
                out.append("    _unused: [u8; 0],")
            else:
                for i, f in enumerate(s["fields"]):
                    fname = sanitize_ident(f["name"], i)
                    out.append(f"    pub {fname}: {self.rust_type(f['cType'])},")
            out.append("}")
        return "\n".join(out)

    def emit_functions(self) -> str:
        out = []
        for fn in self.functions:
            params = []
            for i, p in enumerate(fn["params"]):
                pname = sanitize_ident(p.get("name", ""), i)
                params.append(f"{pname}: {self.rust_type(p['cType'])}")
            ret_c = fn["returnType"]["c"].strip()
            arrow = "" if ret_c == "void" else f" -> {self.rust_type(ret_c)}"
            fname = fn["name"]
            if fname in RUST_KEYWORDS:
                fname = f"r#{fname}"
            out.append('extern "C" {')
            out.append(f"    pub fn {fname}({', '.join(params)}){arrow};")
            out.append("}")
        return "\n".join(out)

    def generate(self) -> str:
        # Map structs / functions first so `self.opaque` is complete before the
        # opaque block is emitted.
        structs = self.emit_structs()
        functions = self.emit_functions()
        enums = self.emit_enums()
        aliases = self.emit_aliases()
        macros = self.emit_macros()
        opaque = self.emit_opaque()

        return "\n".join([
            "// @generated by codegen.py from the MEOS-API catalog "
            "(meos-idl.json). Do not edit by hand.",
            "//",
            "// The catalog is the single source of truth for every MEOS "
            "binding; this",
            "// FFI is a pure projection of it. Regenerate with "
            "`python3 codegen.py`.",
            "",
            "// --- base-type vocabulary ---",
            aliases,
            "",
            "// --- external opaque types ---",
            opaque,
            "",
            "// --- enums ---",
            enums,
            "",
            "// --- #define constants ---",
            macros,
            "",
            "// --- structs ---",
            structs,
            "",
            "// --- functions ---",
            functions,
            "",
        ])


def main() -> None:
    if len(sys.argv) != 3:
        sys.exit("usage: codegen.py <meos-idl.json> <out.rs>")
    idl = json.loads(Path(sys.argv[1]).read_text())
    out = Emitter(idl).generate()
    Path(sys.argv[2]).write_text(out)
    print(f"wrote {sys.argv[2]}")


if __name__ == "__main__":
    main()
