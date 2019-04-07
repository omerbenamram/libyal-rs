extern crate bindgen;

use failure::{bail, Error};
use flate2::read::GzDecoder;
use reqwest;
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tar::Archive;

static LIBBFIO_TAR_GZ_URL: &'static str =
    "https://github.com/libyal/libbfio/releases/download/20190112/libbfio-alpha-20190112.tar.gz";
static LIBBFIO_EXPECTED_DIR_NAME: &'static str = "libbfio-20190112";

fn download_libbfio() -> Result<PathBuf, Error> {
    let temp = PathBuf::from(env::var("OUT_DIR").unwrap());
    let expected_path = temp.join(LIBBFIO_EXPECTED_DIR_NAME);

    // rust can cache the build directory for us when developing
    if !expected_path.exists() {
        println!("Downloading LIBBFIO: from '{}'", LIBBFIO_TAR_GZ_URL);
        let mut response = reqwest::get(LIBBFIO_TAR_GZ_URL)?;

        let (mut dest, p) = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            let fname = temp.join(fname);
            (File::create(&fname)?, fname)
        };

        io::copy(&mut response, &mut dest)?;

        let tar_gz = File::open(p)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(&temp)?;
    }

    if !expected_path.exists() {
        bail!(
            "Expected to find `{}` at `{}`",
            LIBBFIO_EXPECTED_DIR_NAME,
            temp.display()
        );
    }

    Ok(expected_path)
}

fn build_static() {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_STATIC_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        download_libbfio().expect("Failed to download libbfio")
    };

    let target = libbfio.join("dist");

    println!("building with prefix={}", target.display());

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
        target.join("lib").canonicalize().unwrap().display()
    );
}

fn link_dynamic() {
    if let Ok(location) = env::var("LIBBFIO_DYNAMIC_LIBPATH") {
        assert!(
            PathBuf::from(&location).exists(),
            "path passed in LIBBFIO_DYNAMIC_LIBPATH does not exist!"
        );
        println!("cargo:rustc-link-search=native={}", location);
    }
    println!("cargo:rustc-link-lib=dylib=bfio");
}

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(&[
            "-Ilibbfio",
            "-Ilibbfio/common",
            "-Ilibbfio/include",
            "-Ilibbfio/common",
            "-Ilibbfio/libcerror",
        ])
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if cfg!(feature = "static_link") {
        println!("Building static bindings");
        return build_static();
    } else {
        println!("Building dynamic bindings");
        return link_dynamic();
    }
}
