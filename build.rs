extern crate bindgen;

#[cfg(not(windows))]
use pkg_config;
use std::env;
use std::path::PathBuf;
#[cfg(windows)]
use vcpkg;

// const MINIMUM_LEPT_VERSION: &str = "1.80.0";

#[cfg(windows)]
fn find_leptonica_system_lib() -> Option<String> {
    let lib = vcpkg::Config::new().find_package("leptonica").unwrap();

    let include = lib
        .include_paths
        .iter()
        .map(|x| x.to_string_lossy())
        .collect::<String>();
    Some(include)
}

#[cfg(not(windows))]
fn find_leptonica_system_lib() -> Option<String> {
    let pk = pkg_config::Config::new()
        // .atleast_version(MINIMUM_LEPT_VERSION)
        .statik(cfg!(feature="enable-static"))
        .probe("lept")
        .unwrap();
    // Tell cargo to tell rustc to link the system proj shared library.
    for path in pk.link_paths {
        println!("cargo:rustc-link-search=native={:?}", path);
    }
    for lib in pk.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    let mut include_path = pk.include_paths[0].clone();
    // The include file used in this project has "leptonica" as part of
    // the header file already
    include_path.pop();
    Some(include_path.to_string_lossy().to_string())
}

fn main() {
    let clang_extra_include = find_leptonica_system_lib();

    let mut bindings = bindgen::Builder::default().header("wrapper.h");

    if let Some(include_path) = clang_extra_include {
        bindings = bindings.clang_arg(format!("-I{}", include_path));
    }

    let bindings = bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
