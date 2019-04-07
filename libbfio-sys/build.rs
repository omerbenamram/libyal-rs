extern crate bindgen;

use failure::{bail, Error};
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
fn build_static() -> Option<PathBuf> {
    let libbfio = if let Ok(local_install) = env::var("LIBBFIO_STATIC_LIBPATH") {
        PathBuf::from(local_install)
    } else {
        env::current_dir().unwrap().join("libbfio")
    };

    let python_exec =
        env::var("PYTHON_SYS_EXECUTABLE ").unwrap_or_else(|_| "python.exe".to_owned());

    Command::new("powershell")
        .arg("-File")
        .arg("synclibs.ps1")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("synclibs failed");

    Command::new("powershell")
        .arg("-File")
        .arg("autogen.ps1")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("autogen failed");

    std::fs::remove_dir_all(&libbfio.join("vs2015")).unwrap();

    Command::new(&python_exec)
        .arg("..\\..\\vstools\\scripts\\msvscpp-convert.py")
        .arg("--extend-with-x64")
        .arg("--output-format")
        .arg("2015")
        .arg("msvscpp\\libbfio.sln")
        .current_dir(&libbfio)
        .env("PYTHONPATH", "..\\..\\vstools")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("Converting the solution failed. You might need to set PYTHON_SYS_EXECUTABLE to a working python interpreter");

    let target = env::var("TARGET").unwrap();

    let mut msbuild =
        cc::windows_registry::find(&target, "msbuild").expect("Needs msbuild installed");

    msbuild
        .arg("vs2015\\libbfio.sln")
        .arg("/property:PlatformToolset=v141")
        .current_dir(&libbfio)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("Building the solution failed");

    let build_dir = libbfio.join("vs2015").join("Release");

    let build_dir = if target.contains("x86_64") {
        build_dir.join("x64")
    } else {
        build_dir.join("Win32")
    };

    assert!(build_dir.exists(), "Expected {:?} to exist", build_dir);

    // Override h files with bad encoding (anything created by autogen.ps1).
    let lib_h_file = libbfio.join("include").join("libbfio.h");
    utf16le_to_utf8(&lib_h_file).unwrap();

    let types_h_file = libbfio.join("common").join("types.h");
    utf16le_to_utf8(&types_h_file).unwrap();

    for file in ["definitions.h", "features.h", "types.h"].iter() {
        let file_path = libbfio.join("include").join("libbfio").join(file);

        utf16le_to_utf8(&file_path).unwrap();
    }

    println!("cargo:rustc-link-lib=static=libbfio");
    println!(
        "cargo:rustc-link-search=native={}",
        build_dir.to_string_lossy()
    );

    Some(libbfio)
}

#[cfg(target_os = "windows")]
fn utf16le_to_utf8(file_path: &PathBuf) -> Result<(), Error> {
    let h_file = File::open(&file_path)?;

    let mut transcoded = encoding_rs_io::DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_16LE))
        .build(h_file);

    let mut content = String::new();

    transcoded.read_to_string(&mut content)?;

    drop(transcoded);

    let mut h_file = File::create(&file_path)?;
    h_file.write(content.as_bytes())?;

    Ok(())
}

fn link_dynamic() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=libbfio");
    } else {
        println!("cargo:rustc-link-lib=dylib=bfio");
    }

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
