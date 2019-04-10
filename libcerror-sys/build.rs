use failure::{bail, Error};
use libyal_rs_common_build::{build_lib, generate_bindings};
use std::env;
use std::path::PathBuf;

fn build_and_link_static() -> PathBuf {
    let libcerror = if let Ok(local_install) = env::var("LIBCERROR_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libcerror")
    };

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libcerror");

        // Also static-link deps (otherwise we'll get missing symbols at link time).
        println!("cargo:rustc-link-lib=static=libcerror");
        println!("cargo:rustc-link-lib=static=libcdata");
        println!("cargo:rustc-link-lib=static=libcthreads");
    } else {
        println!("cargo:rustc-link-lib=static=cerror");
    }

    build_lib(libcerror, false)
}

fn build_and_link_dynamic() -> PathBuf {
    let libcerror = if let Ok(local_install) = env::var("LIBCERROR_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libcerror")
    };

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libcerror");
    } else {
        println!("cargo:rustc-link-lib=dylib=cerror");
    }

    build_lib(libcerror, true)
}

fn main() {
    // We ignore changes to the C library because it is always changed by the build process.
    println!("cargo:rerun-if-changed=src");

    let include_folder_path = if cfg!(feature = "dynamic_link") {
        build_and_link_dynamic()
    } else {
        build_and_link_static()
    };

    generate_bindings(&include_folder_path, "wrapper.h");
}
