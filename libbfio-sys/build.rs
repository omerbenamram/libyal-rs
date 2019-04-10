use bindgen;
use failure::{bail, Error};
use std::env;
use std::path::PathBuf;
use libyal_rs_common_build::build_lib;

fn build_and_link_static() -> PathBuf {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libbfio")
    };

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libbfio");

        // Also static-link deps (otherwise we'll get missing symbols at link time).
        println!("cargo:rustc-link-lib=static=libcerror");
        println!("cargo:rustc-link-lib=static=libcdata");
        println!("cargo:rustc-link-lib=static=libcthreads");
    } else {
        println!("cargo:rustc-link-lib=static=bfio");
    }

    build_lib(libbfio, false)
}

fn build_and_link_dynamic() -> PathBuf {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libbfio")
    };

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libbfio");
    } else {
        println!("cargo:rustc-link-lib=dylib=bfio");
    }

    build_lib(libbfio, true)
}

fn main() {
    let include_folder_path = if cfg!(feature = "dynamic_link") {
        println!("Building dynamic bindings");
        build_and_link_dynamic()
    } else {
        println!("Building static bindings");
        build_and_link_static()
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(&[format!("-I{}", include_folder_path.to_string_lossy())])
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
