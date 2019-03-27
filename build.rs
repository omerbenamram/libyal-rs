use failure::{bail, Error};
use flate2::read::GzDecoder;
use reqwest;
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tar::Archive;

static LIBFSNTFS_TAR_GZ_URL: &'static str = "https://github.com/libyal/libfsntfs/releases/download/20190104/libfsntfs-experimental-20190104.tar.gz";
static LIBFSNTFS_EXPECTED_DIR_NAME: &'static str = "libfsntfs-20190104";

fn download_libfsntfs() -> Result<PathBuf, Error> {
    let temp = PathBuf::from(env::var("OUT_DIR").unwrap());
    let expected_path = temp.join(LIBFSNTFS_EXPECTED_DIR_NAME);

    // rust can cache the build directory for us when developing
    if !expected_path.exists() {
        println!("Downloading libfsntfs: from '{}'", LIBFSNTFS_TAR_GZ_URL);
        let mut response = reqwest::get(LIBFSNTFS_TAR_GZ_URL)?;

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
            LIBFSNTFS_EXPECTED_DIR_NAME,
            temp.display()
        );
    }

    Ok(expected_path)
}

fn build_static() {
    let libfsntfs = download_libfsntfs().expect("Failed to download libfsntfs");

    let target = libfsntfs.join("dist");

    println!("building with prefix={}", target.display());

    Command::new("sh")
        .arg("configure")
        .arg("--enable-shared=no")
        .arg(format!("--prefix={}", target.display()))
        .current_dir(&libfsntfs)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("configure failed");

    Command::new("make")
        .current_dir(&libfsntfs)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make failed");

    Command::new("make")
        .arg("install")
        .current_dir(&libfsntfs)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("make install failed");

    assert!(
        target.join("lib").exists(),
        "Expected {} to exist",
        target.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=fsntfs");
    println!(
        "cargo:rustc-link-search=native={}",
        target.join("lib").display()
    );
}

fn link_dynamic() {
    if let Ok(location) = env::var("LIBFSNTFS_DYNAMIC_LIBPATH") {
        assert!(
            PathBuf::from(&location).exists(),
            "path passed in LIBFSNTFS_DYNAMIC_LIBPATH does not exist!"
        );
        println!("cargo:rustc-link-search=native={}", location);
    }
    println!("cargo:rustc-link-lib=dylib=fsntfs");
}

fn main() {
    if cfg!(feature = "static_link") {
        println!("Building static bindings");
        return build_static();
    } else {
        println!("Building dynamic bindings");
        return link_dynamic();
    }
}
