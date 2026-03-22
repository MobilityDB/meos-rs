fn main() {
    // When the `bundled` feature is active, meos-sys emits the path to spatial_ref_sys.csv
    // via `cargo:spatial_ref_sys_csv`. Cargo exposes this to dependent crates as
    // DEP_MEOSSYS_SPATIAL_REF_SYS_CSV. We re-emit it as a compile-time env var so
    // `meos_initialize` can call `meos_set_spatial_ref_sys_csv` with the correct path,
    // avoiding the need to copy the file to /usr/local/share/ (which requires extra permissions in CI).

    // TL;DR: To make CI work
    if let Ok(path) = std::env::var("DEP_MEOS_SPATIAL_REF_SYS_CSV") {
        println!("cargo:rustc-env=MEOS_SPATIAL_REF_SYS_CSV={path}");
    }
}
