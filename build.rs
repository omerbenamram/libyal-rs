use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// For now we expect to find a `libfsntfs` dylib
fn main() {
//    let script_dir = env::current_dir().expect("Failed to get current_dir");
//    let libfsntfs = script_dir.join("libfsntfs");
//    let target = libfsntfs.join("dist");
//
//    Command::new("sh")
//        .arg("autogen.sh")
//        .current_dir(&libfsntfs)
//        .stderr(Stdio::inherit())
//        .stdout(Stdio::inherit())
//        .spawn()
//        .expect("autogen failed");
//
//    Command::new("sh")
//        .arg("configure")
//        .arg("--enable-shared=no")
//        .arg(format!("--prefix={}", target.to_str().unwrap()))
//        .current_dir(&libfsntfs)
//        .stderr(Stdio::inherit())
//        .stdout(Stdio::inherit())
//        .spawn()
//        .expect("configure failed");
//
//    Command::new("make install")
//        .current_dir(&libfsntfs)
//        .stderr(Stdio::inherit())
//        .stdout(Stdio::inherit())
//        .spawn()
//        .expect("make failed");
//
//    Command::new("make install")
//        .current_dir(&libfsntfs)
//        .stderr(Stdio::inherit())
//        .stdout(Stdio::inherit())
//        .spawn()
//        .expect("make install failed");

    println!("cargo:rustc-link-lib=dylib=fsntfs");
    println!("cargo:rustc-link-search=native=./libfsntfs/dist/lib");
}
