use crate::attribute::{Attribute, AttributeType};
use crate::error::Error;
use crate::file_entry::FileEntry;
use crate::volume::{AccessMode, Volume};
use env_logger;
use lazy_static::lazy_static;
use std::path::PathBuf;
use libbfio_rs::handle::Handle;

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
        .parent()
        .unwrap()
        .join("examples")
        .join("ntfs-img-kw-1.dd");

    sample.to_str().unwrap().to_string()
}

pub fn sample_volume_io_handle() -> Result<Handle, Error> {
    let volume_path = sample_volume_path();
    Ok(Handle::open_file(volume_path).expect("libbfio failed"))
}

pub fn sample_volume() -> Result<Volume, Error> {
    let volume_path = sample_volume_path();
    Volume::open(&volume_path, AccessMode::Read)
}

pub fn entries_with_data(volume: &Volume) {
    let entries: Vec<FileEntry> = volume
        .iter_entries()
        .unwrap()
        .filter_map(|f| f.ok())
        .collect();

    for (i, entry) in entries.iter().enumerate() {
        if let Ok(enumerator) = entry.iter_attributes() {
            let alloc: Vec<Attribute> = enumerator.filter_map(|attr| attr.ok()).collect();

            let has_filename = alloc
                .iter()
                .find(|a| a.get_type().unwrap() == AttributeType::FileName)
                .is_some();

            let has_data = alloc
                .iter()
                .find(|a| a.get_type().unwrap() == AttributeType::Data)
                .is_some();

            if has_filename && has_data {
                println!("{}: {:?}", i, alloc);
            }
        }
    }
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
                Ok(attrs) => {
                    let alloc: Vec<Attribute> = attrs.filter_map(|attr| attr.ok()).collect();
                    let has_filename = alloc
                        .iter()
                        .find(|a| a.get_type().unwrap() == AttributeType::FileName)
                        .is_some();

                    let has_data = alloc
                        .iter()
                        .find(|a| a.get_type().unwrap() == AttributeType::Data)
                        .is_some();

                    has_filename && has_data
                }
                Err(_) => false,
            }
        })
        .unwrap();

    Ok(f)
}
