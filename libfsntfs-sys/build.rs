use failure::{bail, Error};
use libyal_rs_common_build::{build_lib, generate_bindings};
use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::PathBuf;

fn build_and_link_static() -> PathBuf {
    let libfsntfs = if let Ok(local_install) = env::var("LIBFSNTFS_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libfsntfs")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libfsntfs");

    copy(libfsntfs, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=libfsntfs");

        // Also static-link deps (otherwise we'll get missing symbols at link time).
        let deps = [
            "libbfio",
            "libcdata",
            "libcerror",
            "libcfile",
            "libclocale",
            "libcnotify",
            "libcpath",
            "libcsplit",
            "libcthreads",
            "libfcache",
            "libfdata",
            "libfdatetime",
            "libfguid",
            "libfusn",
            "libfwnt",
            "libuna",
        ];

        for dep in deps.iter() {
            println!("cargo:rustc-link-lib=static={}", dep);
        }
    } else {
        println!("cargo:rustc-link-lib=static=fsntfs");
    }

    build_lib(out_path, false)
}

fn build_and_link_dynamic() -> PathBuf {
    let libfsntfs = if let Ok(local_install) = env::var("LIBFSNTFS_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libfsntfs")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libfsntfs");

    copy(libfsntfs, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libfsntfs");
    } else {
        println!("cargo:rustc-link-lib=dylib=fsntfs");
    }

    build_lib(out_path, true)
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
