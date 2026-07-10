# meos-sys

Low level [MEOS](https://libmeos.org/) C API bindings for MEOS.

It provides C-interface as is. If you want to use a more Rust-friendly crate,
use the [meos](https://github.com/MobilityDB/meos-rs) crate.

You can also find it on [crates.io](https://crates.io/crates/meos).

Versions >= 1.1 are supported.

## Usage

Select a feature depending on your setup:

| Feature    | Description |
|------------|-------------|
| `v1_1`     | Link against a system-installed MEOS. |
| `v1_2`     | Link against a system-installed MEOS. |
| `v1_3`     | Link against a system-installed MEOS. |
| `bundled`  | Build MEOS 1.3 from source. No system MEOS required. |

The FFI itself is the same for every feature: `src/generated.rs`, targeting the
MEOS 1.3 API. `pkg-config` locates a system MEOS; `bundled` builds MEOS and its
dependencies from the bundled source instead.

```toml
# Cargo.toml

# Use system-installed MEOS
meos-sys = { version = "0.1.9", features = ["v1_3"] }

# Build MEOS from source (no system dependency needed)
meos-sys = { version = "0.1.9", features = ["bundled"] }
```

## Build

### System-installed MEOS (`v1_1`, `v1_2`, `v1_3`)

`pkg-config` is used to detect MEOS automatically. For MEOS 1.1 (which predates
pkg-config support), set `MEOS_LIB_DIR` to point to the library directory:

```bash
# Linux
LD_LIBRARY_PATH=<path>/lib MEOS_LIB_DIR=<path>/lib cargo build
```

### Bundled (`bundled`)

Builds MEOS 1.3 and all its dependencies (GEOS, PROJ, JSON-C, GSL) from the
bundled source as static libraries. No system MEOS installation required.

The following system packages are required to compile:

- `cmake` — build system used to compile MEOS and its dependencies (GEOS, PROJ, JSON-C, GSL)
- `pkg-config` — used to locate system libraries during the build
- `sqlite3` / `libsqlite3-dev` — required by PROJ, which uses SQLite to store its coordinate reference system database

```bash
# Debian/Ubuntu
apt-get install cmake pkg-config sqlite3 libsqlite3-dev
```

Additionally, the git submodules (JSON-C, GSL) must be initialized before building, as they are bundled as git submodules rather than downloaded at build time:

```bash
git submodule update --init --recursive
```

> **Note:** If you have a system `libmeos.so` installed (e.g. at
> `/usr/local/lib/libmeos.so`), remove or unload it before using `bundled` to
> avoid conflicts.

Compilation will take longer due to building all dependencies from source.

## Generated bindings

`src/generated.rs` is a projection of the [MEOS-API](https://github.com/MobilityDB/MEOS-API)
catalog `meos-idl.json` — the single source of truth from which every MEOS
binding is derived, and the one place a MEOS header is ever parsed. `codegen.py`
emits the `#[repr(C)]` structs, `extern "C"` functions, enums, and `#define`
constants from that catalog:

```bash
python3 codegen.py <meos-idl.json> src/generated.rs
```
