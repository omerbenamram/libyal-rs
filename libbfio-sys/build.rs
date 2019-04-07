extern crate bindgen;

use failure::{bail, Error};
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};


fn build_static() -> Option<PathBuf> {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_STATIC_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libbfio")
    };

    let target = libbfio.join("dist");

    println!("building with prefix={}", target.display());

    Command::new("sh")
        .arg("synclibs.sh")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("synclibs failed");

    Command::new("sh")
        .arg("autogen.sh")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("autogen failed");

    Command::new("sh")
        .arg("configure")
        .arg("--enable-shared=no")
        .arg(format!("--prefix={}", target.display()))
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("configure failed");

    Command::new("make")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make failed");

    Command::new("make")
        .arg("install")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make install failed");

    assert!(
        target.join("lib").exists(),
        "Expected {} to exist",
        target.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=bfio");
    println!(
        "cargo:rustc-link-search=native={}",
        target.join("lib").canonicalize().unwrap().to_string_lossy()
    );

    Some(target)
}

fn link_dynamic() -> Option<PathBuf> {
    println!("cargo:rustc-link-lib=dylib=bfio");

    if let Ok(location) = env::var("LIBBFIO_DYNAMIC_LIBPATH") {
        assert!(
            PathBuf::from(&location).exists(),
            "path passed in LIBBFIO_DYNAMIC_LIBPATH does not exist!"
        );
        println!("cargo:rustc-link-search=native={}", location);

        return Some(location.into());
    }

    None
}

fn main() {
    let target = if cfg!(feature = "static_link") {
        println!("Building static bindings");
        build_static()
    } else {
        println!("Building dynamic bindings");
        link_dynamic()
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h");

    let builder = if let Some(target) = target {
        builder.clang_args(&[format!("-I{}/include", target.to_string_lossy())])
    } else {
        builder
    };

    let bindings = builder
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
