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
| `v1_1`     | Use prebuilt bindings for MEOS 1.1. Requires MEOS 1.1 installed on your system. |
| `v1_2`     | Use prebuilt bindings for MEOS 1.2. Requires MEOS 1.2 installed on your system. |
| `v1_3`     | Use prebuilt bindings for MEOS 1.3. Requires MEOS 1.3 installed on your system. |
| `bundled`  | Build MEOS 1.3 from source. No system MEOS required. Implies `bindgen`. |
| `bindgen`  | Generate bindings at build time from your system-installed MEOS headers. |

```toml
# Cargo.toml

# Use system-installed MEOS 1.3 with prebuilt bindings
meos-sys = { version = "0.1.9", features = ["v1_3"] }

# Build MEOS from source (no system dependency needed)
meos-sys = { version = "0.1.9", features = ["bundled"] }

# Generate bindings from your system MEOS headers at build time
meos-sys = { version = "0.1.9", features = ["v1_3", "bindgen"] }
```

## Build

### System-installed MEOS (`v1_1`, `v1_2`, `v1_3`)

`pkg-config` is used to detect MEOS automatically. For MEOS 1.1 (which predates
pkg-config support), set `MEOS_LIB_DIR` to point to the library directory:

```bash
# Linux
LD_LIBRARY_PATH=<path>/lib MEOS_LIB_DIR=<path>/lib cargo build

# macOS
DYLD_FALLBACK_LIBRARY_PATH=<path>/lib MEOS_LIB_DIR=<path>/lib cargo build
```

### Bundled (`bundled`)

Builds MEOS 1.3 and all its dependencies (GEOS, PROJ, JSON-C, GSL) from the
bundled source as static libraries. No system MEOS installation required.

> **Note:** If you have a system `libmeos.so` installed (e.g. at
> `/usr/local/lib/libmeos.so`), remove or unload it before using `bundled` to
> avoid conflicts.

Compilation will take longer due to building all dependencies from source.

### Bindgen (`bindgen`)

Generates bindings at build time from your system-installed MEOS headers instead
of using the prebuilt ones. Useful when your installed version differs slightly
from the prebuilt snapshots. Requires `libclang` to be installed.

When `bundled` is active, `bindgen` is implied automatically and generates
bindings from the bundled source headers.
