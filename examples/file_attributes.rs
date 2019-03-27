extern crate libfsntfs_sys;

use libfsntfs_sys::volume::{AccessMode, Volume};

fn main() {
    let sample = String::from("/Users/omerba/Workspace/libfsntfs-sys/examples/ntfs-img-kw-1.dd");
    let volume = Volume::open(&sample, AccessMode::Read).unwrap();

    for entry in volume
        .iter_entries()
        .unwrap()
        .filter_map(|entry| entry.ok())
    {
        println!("{:?}", entry);
        for attribute in entry
            .iter_attributes()
            .unwrap()
            .filter_map(|attr| attr.ok())
        {
            println!("\t{:?}", attribute);
            println!("\t{:#?}", attribute.get_data())
        }
    }
}
