use failure::{bail, Error};
use fs_extra::dir::{copy, CopyOptions};
use libyal_rs_common_build::{sync_and_build_lib, generate_bindings};
use std::env;
use std::path::PathBuf;

fn build_and_link_static() -> PathBuf {
    let libcerror = if let Ok(local_install) = env::var("LIBCERROR_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libcerror")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libcerror");

    copy(&libcerror, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libcerror");
    } else {
        println!("cargo:rustc-link-lib=static=cerror");
    }

    sync_and_build_lib(out_path, false)
}

fn build_and_link_dynamic() -> PathBuf {
    let libcerror = if let Ok(local_install) = env::var("LIBCERROR_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libcerror")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libcerror");
    let _ = std::fs::remove_dir_all(&out_path);

    copy(&libcerror, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libcerror");
    } else {
        println!("cargo:rustc-link-lib=dylib=cerror");
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
