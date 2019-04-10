use failure::{bail, Error};
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Build the lib on posix platforms (using configure and make).
/// This function will also add the needed folder to the `link-search` path.
/// Return the "include" folder for the library (to be used by bindgen).
pub fn build_lib(lib_path: PathBuf, shared: bool) -> PathBuf {
    let target = lib_path.join("dist");

    println!("building with prefix={}", target.display());

    Command::new("sh")
        .arg("synclibs.sh")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("synclibs failed");

    Command::new("sh")
        .arg("autogen.sh")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("autogen failed");

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

    configure_cmd.status().expect("configure failed");

    Command::new("make")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make failed");

    Command::new("make")
        .arg("install")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make install failed");

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
