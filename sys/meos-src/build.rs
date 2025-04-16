fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let libmeos = cmake::Config::new("source")
        .define("MEOS", "1")
        .very_verbose(true)
        .build();
    println!("cargo:lib=meos");
    let search_path = libmeos.display().to_string();
    assert!(std::path::Path::new(&search_path).exists());
    println!("cargo:search={}", search_path);
}
