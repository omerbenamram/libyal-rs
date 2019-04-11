use failure::{bail, Error};
use fs_extra::dir::{copy, CopyOptions};
use libyal_rs_common_build::{build_lib, generate_bindings, sync_libs};
use std::env;
use std::fs::File;
use std::io::{Write, Read};
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
    sync_libs(&libfsntfs);

    build_lib(out_path, false)
}

fn build_and_link_dynamic() -> PathBuf {
    let libfsntfs = if let Ok(local_install) = env::var("LIBFSNTFS_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libfsntfs")
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("libfsntfs");
    let _ = std::fs::remove_dir_all(&out_path);

    copy(libfsntfs, &out_path.parent().unwrap(), &CopyOptions::new())
        .expect("Error while copying sources to `OUT_DIR`");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libfsntfs");
    } else {
        println!("cargo:rustc-link-lib=dylib=fsntfs");
    }

    sync_libs(&out_path);

    // Patch libcache to fix a segfault (See https://github.com/libyal/libfsntfs/issues/10).
    let patched_file_path = out_path.join("libcache").join("libfcache_cache_value.c");
    let mut org_file_content = String::new();

    File::open(&patched_file_path)
        .unwrap()
        .read_to_string(&mut org_file_content)
        .unwrap();

    let patched_file_lines: Vec<&str> = org_file_content.lines().enumerate()
        .filter(|(line_idx, line)| (line_idx + 1 < 477) && (489 < line_idx +1) )
        .map(|(line_idx, line)| line)
        .collect();

    let patched_file_content = patched_file_lines.join("\n");

    File::create(&patched_file_path)
        .unwrap()
        .write_all(&patched_file_content.as_bytes())
        .unwrap();

    build_lib(libfsntfs, true)
}

fn main() {
    let include_folder_path = if cfg!(feature = "dynamic_link") {
        build_and_link_dynamic()
    } else {
        build_and_link_static()
    };

    generate_bindings(&include_folder_path, "wrapper.h");
}
