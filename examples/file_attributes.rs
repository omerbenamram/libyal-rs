extern crate libfsntfs_sys;


use libfsntfs_sys::volume::{Volume, AccessMode};
use std::path::PathBuf;


fn main() {
    let sample = String::from("/Users/omerba/Workspace/libfsntfs-sys/examples/ntfs-img-kw-1.dd");
    let mut volume = Volume::open(&sample, AccessMode::Read).unwrap();

//    let mut entry = volume.get_file_entry_by_path("/").unwrap();

    println!("{}", volume.get_name().unwrap())
}