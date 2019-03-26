use crate::error::Error;
use crate::file_entry::FileEntry;
use crate::volume::{AccessMode, Volume};
use env_logger;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref LOGGER_INIT: () = {
        env_logger::init();
    };
}

pub fn sample_volume_path() -> String {
    let this_file = file!();
    let sample = PathBuf::from(this_file)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
        .join("ntfs-img-kw-1.dd");

    sample.to_str().unwrap().to_string()
}

pub fn sample_volume() -> Result<Volume, Error> {
    let volume_path = sample_volume_path();
    Volume::open(&volume_path, AccessMode::Read)
}

pub fn file_entry() -> Result<FileEntry, Error> {
    let volume = sample_volume().expect("Sample volume fixture should work");
    volume.iter_entries().unwrap().take(1).next().unwrap()
}
