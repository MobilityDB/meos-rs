fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Build json-c
    let json_c_path = cmake::Config::new("json-c")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .build();

    // Build gsl
    let gsl_path = cmake::Config::new("gsl")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("NO_AMPL_BINDINGS", "1")
        .build();

    let mut config = cmake::Config::new("source");
    config.define("MEOS", "1").very_verbose(true).define("BUILD_SHARED_LIBS", "OFF");

    // Collect include paths from dependencies
    let mut prefix_paths = Vec::new();
    
    prefix_paths.push(json_c_path.display().to_string());
    prefix_paths.push(gsl_path.display().to_string());

    // geos-sys exports "includedir" which becomes DEP_GEOS_INCLUDEDIR
    let geos_include_env = std::env::var("DEP_GEOS_INCLUDEDIR")
        .or_else(|_| std::env::var("DEP_GEOS_INCLUDE"));

    if let Ok(geos_root) = std::env::var("DEP_GEOSSRC_ROOT") {
        let root_path = std::path::Path::new(&geos_root);
        let include_path = root_path.join("include");
        let lib_path = root_path.join("lib").join("libgeos_c.a");
        
        config.define("GEOS_INCLUDE_DIR", include_path);
        config.define("GEOS_LIBRARY", lib_path);
        prefix_paths.push(geos_root);
    } else if let Ok(geos_include) = geos_include_env {
        // geos-sys often returns just the include dir. We need the root for CMAKE_PREFIX_PATH usually.
        // Or we can set GEOS_INCLUDE_DIR directly.
        config.define("GEOS_INCLUDE_DIR", &geos_include);
        // Try to infer root from include (../)
        let path = std::path::Path::new(&geos_include);
        if let Some(parent) = path.parent() {
            // The structure is usually .../out/lib/include or .../out/include
            // We want to find the directory that contains "lib"
            // If path is .../out/lib/include, parent is .../out/lib
            
            // Check if parent contains lib
            if parent.join("libgeos_c.a").exists() {
                 config.define("GEOS_LIBRARY", parent.join("libgeos_c.a").as_os_str());
                 if let Some(grandparent) = parent.parent() {
                     prefix_paths.push(grandparent.to_string_lossy().to_string());
                 }
            } else {
                 prefix_paths.push(parent.to_string_lossy().to_string());
                 
                 // Try to find the library explicitly
                // Check for libgeos_c.a or libgeos_c.so or similar
                // Since we are building statically via geos-sys, it should be libgeos_c.a
                let lib_path = parent.join("lib").join("libgeos_c.a");
                 if lib_path.exists() {
                     config.define("GEOS_LIBRARY", lib_path.as_os_str());
                } else {
                    // If not found in standard location, let's look in build/lib as seen in find command
                     let build_lib_path = parent.join("build").join("lib").join("libgeos_c.a");
                     if build_lib_path.exists() {
                         config.define("GEOS_LIBRARY", build_lib_path.as_os_str());
                     }
                }
            }
        }
    }

    if let Ok(proj_root) = std::env::var("DEP_PROJ_ROOT") {
        let root_path = std::path::Path::new(&proj_root);
        let include_path = root_path.join("include");
        let lib_path = root_path.join("lib").join("libproj.a");
        
        config.define("PROJ_INCLUDE_DIRS", include_path);
        config.define("PROJ_LIBRARIES", lib_path);
        prefix_paths.push(proj_root);
    } else if let Ok(proj_include) = std::env::var("DEP_PROJ_INCLUDE") {
        config.define("PROJ_INCLUDE_DIRS", &proj_include); // MEOS CMake might look for this

        // Try to infer root
        let path = std::path::Path::new(&proj_include);
        if let Some(parent) = path.parent() {
            prefix_paths.push(parent.to_string_lossy().to_string());

             // Try to find the library explicitly
             let lib_path = parent.join("lib").join("libproj.a");
             if lib_path.exists() {
                 config.define("PROJ_LIBRARIES", lib_path.as_os_str());
            } else {
                 let build_lib_path = parent.join("build").join("lib").join("libproj.a");
                 if build_lib_path.exists() {
                     config.define("PROJ_LIBRARIES", build_lib_path.as_os_str());
                 }
            }
        }
    }
    
    // Pass libraries explicitly if available (for static linking mostly)
    // NOTE: This is tricky with CMake's find_package. 
    // Setting CMAKE_PREFIX_PATH is the most standard way to help find_package.
    if !prefix_paths.is_empty() {
        let joined_paths = prefix_paths.join(";"); // CMake list separator
        config.define("CMAKE_PREFIX_PATH", joined_paths);
    }

    let libmeos = config.build();

    println!("cargo:lib=meos");
    let search_path = libmeos.display().to_string();
    assert!(std::path::Path::new(&search_path).exists());
    println!("cargo:search={}", search_path);

    // Link JSON-C and GSL static libraries
    println!("cargo:rustc-link-search=native={}/lib", json_c_path.display());
    println!("cargo:rustc-link-search=native={}/lib64", json_c_path.display()); // Some distros/CMake use lib64
    println!("cargo:rustc-link-lib=static=json-c");

    println!("cargo:rustc-link-search=native={}/lib", gsl_path.display());
    println!("cargo:rustc-link-search=native={}/lib64", gsl_path.display());
    println!("cargo:rustc-link-lib=static=gsl");
    println!("cargo:rustc-link-lib=static=gslcblas");
}
