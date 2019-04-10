use failure::{bail, Error};
use libyal_rs_common_build::{build_lib, generate_bindings};
use std::env;
use std::path::PathBuf;

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

    generate_bindings(&include_folder_path, "wrapper.h");
}
