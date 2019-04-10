#[cfg(not(target_os = "windows"))]
mod posix;

#[cfg(not(target_os = "windows"))]
pub use crate::posix::build_lib;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use crate::windows::build_lib;

use std::env;
use std::path::PathBuf;

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
