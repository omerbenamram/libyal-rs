#[cfg(not(target_os = "windows"))]
mod posix;

#[cfg(not(target_os = "windows"))]
pub use crate::posix::{build_lib, sync_libs};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use crate::windows::{build_lib, sync_libs};

use fs_extra::dir::{copy, CopyOptions};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::{create_dir, create_dir_all};
use std::path::PathBuf;

/// Sync dependencies and build the lib.
/// See `build_lib` for more.
pub fn sync_and_build_lib(lib_path: PathBuf, shared: bool) -> PathBuf {
    sync_libs(&lib_path);

    build_lib(lib_path, shared)
}

/// Find the library (based on env var or using the local submodule),
/// copy it to the output folder and return the copied folder's path.
pub fn get_lib_and_copy_to_out_dir(lib_name: &str) -> PathBuf {
    let lib_path =
        if let Ok(local_install) = env::var(format!("{}_LIBPATH", lib_name.to_uppercase())) {
            PathBuf::from(local_install)
        } else {
            // For each `-sys` package, we expect the lib to be next to the Cargo.toml file.
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(lib_name)
        };

    let rand_folder_name: String = thread_rng().sample_iter(&Alphanumeric).take(6).collect();

    // Output dir might be quite nested, so use wide paths instead of regular paths.
    let build_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // We have to use a random build dir because multiple builds of the same lib might happen at the same time.
    let random_build_dir = build_out_dir.join(rand_folder_name);

    create_dir_all(&random_build_dir).unwrap();

    let copied_lib_path = random_build_dir.join(lib_name);
    let _ = std::fs::remove_dir_all(&copied_lib_path);

    copy(&lib_path, &random_build_dir, &CopyOptions::new())
    .expect(&format!(
        "Error while copying sources from {:?} to `OUT_DIR` {:?}",
        &lib_path, &random_build_dir
    ));

    copied_lib_path
}

pub fn generate_bindings(include_folder_path: &PathBuf, header_file_name: &str) {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(&[format!("-I{}", include_folder_path.to_string_lossy())])
        .header(header_file_name)
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
