use failure::{bail, Error};
use fs_extra::dir::{copy, CopyOptions};
use libyal_rs_common_build::{sync_and_build_lib, generate_bindings};
use std::env;
use std::path::PathBuf;

fn build_and_link_static() -> PathBuf {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libbfio")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libbfio");

    copy(&libbfio, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libbfio");

        // Also static-link deps (otherwise we'll get missing symbols at link time).
        println!("cargo:rustc-link-lib=static=libcerror");
        println!("cargo:rustc-link-lib=static=libcdata");
        println!("cargo:rustc-link-lib=static=libcthreads");
    } else {
        println!("cargo:rustc-link-lib=static=bfio");
    }

    sync_and_build_lib(out_path, false)
}

fn build_and_link_dynamic() -> PathBuf {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libbfio")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libbfio");
    let _ = std::fs::remove_dir_all(&out_path);

    copy(&libbfio, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libbfio");
    } else {
        println!("cargo:rustc-link-lib=dylib=bfio");
    }

    sync_and_build_lib(out_path, true)
}

fn main() {
    let include_folder_path = if cfg!(feature = "dynamic_link") {
        build_and_link_dynamic()
    } else {
        build_and_link_static()
    };

    generate_bindings(&include_folder_path, "wrapper.h");
}
