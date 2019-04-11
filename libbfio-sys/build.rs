use failure::{bail, Error};
use libyal_rs_common_build::{sync_and_build_lib, generate_bindings, get_lib_and_copy_to_out_dir};
use std::env;
use std::path::PathBuf;

fn build_and_link_static(lib_path: PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libbfio");

        // Also static-link deps (otherwise we'll get missing symbols at link time).
        println!("cargo:rustc-link-lib=static=libcerror");
        println!("cargo:rustc-link-lib=static=libcdata");
        println!("cargo:rustc-link-lib=static=libcthreads");
    } else {
        println!("cargo:rustc-link-lib=static=bfio");
    }

    sync_and_build_lib(lib_path, false)
}

fn build_and_link_dynamic(lib_path: PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libbfio");
    } else {
        println!("cargo:rustc-link-lib=dylib=bfio");
    }

    sync_and_build_lib(lib_path, true)
}

fn main() {
    let lib_path = get_lib_and_copy_to_out_dir("libbfio");

    let include_folder_path = if cfg!(feature = "dynamic_link") {
        build_and_link_dynamic(lib_path)
    } else {
        build_and_link_static(lib_path)
    };

    generate_bindings(&include_folder_path, "wrapper.h");
}
