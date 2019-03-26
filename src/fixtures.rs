use crate::attribute::{Attribute, AttributeType};
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

pub fn file_entry(volume: &Volume) -> Result<FileEntry, Error> {
    let entries: Vec<FileEntry> = volume
        .iter_entries()
        .unwrap()
        .filter_map(|f| f.ok())
        .collect();

    let f = entries
        .into_iter()
        .find(|f| {
            let e = f.iter_attributes();
            match e {
                Ok(attrs) => attrs
                    .filter_map(|attr| attr.ok())
                    .find(|a| a.get_type().unwrap() == AttributeType::FileName)
                    .is_some(),
                Err(_) => false,
            }
        })
        .unwrap();

    Ok(f)
}
