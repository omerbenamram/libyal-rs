use failure::{bail, Error};
use libyal_rs_common_build::{build_lib, generate_bindings, sync_libs, get_lib_and_copy_to_out_dir};
use std::env;
use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;

fn build_and_link_static(lib_path: PathBuf) -> PathBuf {
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

    build_lib(lib_path, false)
}

fn build_and_link_dynamic(lib_path: PathBuf) -> PathBuf {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libfsntfs");
    } else {
        println!("cargo:rustc-link-lib=dylib=fsntfs");
    }

    build_lib(lib_path, true)
}

fn main() {
    let lib_path = get_lib_and_copy_to_out_dir("libfsntfs");

    sync_libs(&lib_path);

    // Patch libfcache to fix a segfault (See https://github.com/libyal/libfsntfs/issues/10).
    let patched_file_path = lib_path.join("libfcache").join("libfcache_cache_value.c");
    let mut org_file_content = String::new();

    File::open(&patched_file_path)
        .unwrap()
        .read_to_string(&mut org_file_content)
        .unwrap();

    let patched_file_lines: Vec<&str> = org_file_content.lines().enumerate()
        .filter(|(line_idx, _line)| (line_idx + 1 < 477) || (489 < line_idx +1) )
        .map(|(_line_idx, line)| line)
        .collect();

    let patched_file_content = patched_file_lines.join("\n");

    File::create(&patched_file_path)
        .unwrap()
        .write_all(&patched_file_content.as_bytes())
        .unwrap();

    let include_folder_path = if cfg!(feature = "dynamic_link") {
        build_and_link_dynamic(lib_path)
    } else {
        build_and_link_static(lib_path)
    };

    generate_bindings(&include_folder_path, "wrapper.h");
}
