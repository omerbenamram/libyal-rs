use failure::{bail, Error};
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Synchronizes the local library dependencies.
pub fn sync_libs(lib_path: &PathBuf) {
    let status = Command::new("sh")
        .arg("synclibs.sh")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("synclibs failed");

    assert!(status.success(), "synclibs failed");
}

/// Build the lib on posix platforms (using configure and make).
/// Note, this function will not sync dependencies. use `sync_libs` or `sync_and_build_lib`.
/// This function will also add the needed folder to the `link-search` path.
/// Return the "include" folder for the library (to be used by bindgen).
pub fn build_lib(lib_path: PathBuf, shared: bool) -> PathBuf {
    let target = lib_path.join("dist");

    let status = Command::new("sh")
        .arg("autogen.sh")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("autogen failed");

    assert!(status.success(), "autogen failed");

    let mut configure_cmd = Command::new("sh");

    configure_cmd
        .arg("configure")
        .arg(format!("--prefix={}", target.display()))
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit());

    if !shared {
        configure_cmd.arg("--enable-shared=no");
    }

    let status = configure_cmd.status().expect("configure failed");

    assert!(status.success(), "configure failed");

    let status = Command::new("make")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make failed");

    assert!(status.success(), "make failed");

    let status = Command::new("make")
        .arg("install")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make install failed");

    assert!(status.success(), "make install failed");

    assert!(
        target.join("lib").exists(),
        "Expected {} to exist",
        target.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        target.join("lib").canonicalize().unwrap().to_string_lossy()
    );

    target.join("include")
}
