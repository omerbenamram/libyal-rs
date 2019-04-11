use encoding_rs_io::DecodeReaderBytesBuilder;
use failure::{bail, Error};
use std::env;
use std::fs::{remove_dir_all, File};
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

/// Synchronizes the local library dependencies.
pub fn sync_libs(lib_path: &PathBuf) {
    let status = Command::new("powershell")
        .arg("-File")
        .arg("synclibs.ps1")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("synclibs failed");

    assert!(status.success(), "synclibs failed");
}

/// Build the lib on windows (using msbuild and libyal's vstools).
/// Note, this function will not sync dependencies. use `sync_libs` or `sync_and_build_lib`.
/// Require python to be installed.
/// This function will also add the needed folder to the `link-search` path.
/// Return the "include" folder for the library (to be used by bindgen).
pub fn build_lib(lib_path: PathBuf, shared: bool) -> PathBuf {
    let python_exec = env::var("PYTHON_SYS_EXECUTABLE").unwrap_or_else(|_| "python.exe".to_owned());

    let status = Command::new("powershell")
        .arg("-File")
        .arg("autogen.ps1")
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()
        .expect("autogen failed");

    assert!(status.success(), "autogen failed");

    // The folder might not exists from a previous build, but we don't care.
    let _ = remove_dir_all(&lib_path.join("vs2015"));

    let vstools_path = PathBuf::from(file!()).parent().unwrap().parent().unwrap().parent().unwrap().join("vstools");

    let lib_name = lib_path.file_name().unwrap().to_string_lossy().into_owned();

    let py_convert_status = Command::new(&python_exec)
        .arg(vstools_path.join("scripts").join("msvscpp-convert.py"))
        .arg("--extend-with-x64")
        .arg("--output-format")
        .arg("2015")
        .arg(format!("msvscpp\\{}.sln", lib_name))
        .current_dir(&lib_path)
        .env("PYTHONPATH", vstools_path.into_os_string())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status();

    match py_convert_status {
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                panic!(format!(
                    "Could not find python at {}, \
                     You might need to set PYTHON_SYS_EXECUTABLE: {:?}",
                    python_exec, err
                ));
            } else {
                panic!(format!("Failed to convert the solution: {:?}", err));
            }
        }
        Ok(status) => {
            assert!(status.success(), "Failed to convert the solution");
        }
    };

    let target = env::var("TARGET").unwrap();

    let mut msbuild =
        cc::windows_registry::find(&target, "msbuild").expect("Needs msbuild installed");

    let msbuild_platform = if target.contains("x86_64") {
        "x64"
    } else {
        "Win32"
    };

    msbuild
        .arg(format!("vs2015\\{}.sln", lib_name))
        .arg("/p:PlatformToolset=v141")
        .arg(format!("/p:Platform={}", msbuild_platform))
        .current_dir(&lib_path)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit());

    if !shared {
        msbuild.arg("/p:ConfigurationType=StaticLibrary");
    }

    // We do not check status here because the Python bindings might failed to build,
    // but we don't care about that.
    let _status = msbuild.status().expect("Building the solution failed");

    let build_dir = lib_path
        .join("vs2015")
        .join("Release")
        .join(msbuild_platform);

    assert!(build_dir.exists(), "Expected {:?} to exist", build_dir);

    println!(
        "cargo:rustc-link-search=native={}",
        build_dir.to_string_lossy()
    );

    // h files created by autogen.ps1 (`.in.h` -> `.h`) are UTF16LE encoded,
    // which llvm (and therefore bindgen) does not accept.
    // So convert them back to UTF8.
    let autogen_dirs: Vec<PathBuf> = ["common", "include", &lib_name]
        .into_iter()
        .map(|dir_name| lib_path.join(dir_name))
        .collect();

    for file_entry in autogen_dirs.iter().map(WalkDir::new).flatten() {
        let file_entry = file_entry.unwrap();
        let file_path = file_entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy();

        if !file_name.ends_with(".h.in") {
            continue;
        }

        let h_file_path = file_path.with_file_name(file_name.replace(".h.in", ".h"));

        utf16le_to_utf8(&h_file_path).unwrap();
    }

    let include_folder_path = lib_path.join("include");

    include_folder_path
}

fn utf16le_to_utf8(file_path: &PathBuf) -> Result<(), Error> {
    let h_file = File::open(&file_path)?;

    let mut transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_16LE))
        .build(h_file);

    let mut content = String::new();

    transcoded.read_to_string(&mut content)?;

    drop(transcoded);

    let mut h_file = File::create(&file_path)?;
    h_file.write(content.as_bytes())?;

    Ok(())
}
