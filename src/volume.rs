use crate::file_entry::FileEntry;
use crate::libfsntfs::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::ptr;

pub struct Volume {
    volume: isize,
}

pub type MftEntryIndex = u64;

impl Volume {
    /// Opens a volume.
    fn open(self, filename: impl AsRef<&str>, mode: &str) {
        unimplemented!();
    }

    /// Closes a volume.
    fn close(self) {
        unimplemented!();
    }

    /// Retrieves a specific file entry.
    fn get_file_entry(self, idx: MftEntryIndex) -> Result<FileEntry> {
        let mut file_entry = 0;
        let mut error = ptr::null_mut();
        unsafe {
            libfsntfs_volume_get_file_entry_by_index(
                &mut self.volume as *mut _,
                idx,
                &mut file_entry as *mut _,
                &mut error as *mut _,
            );
        }

        if error.is_null() {
            Ok(FileEntry { file_entry })
        }
    }

    /// Retrieves a file entry specified by the path.
    fn get_file_entry_by_path(self, path: impl AsRef<Path>) {
        unimplemented!();
    }

    /// Retrieves the name.
    fn get_name(self) {
        unimplemented!();
    }

    /// Retrieves the number of file entries.
    fn get_number_of_file_entries(self) {
        unimplemented!();
    }

    /// Retrieves the root directory.
    fn get_root_directory(self) {
        unimplemented!();
    }

    /// Retrieves the USN change journal.
    fn get_usn_change_journal(self) {
        unimplemented!();
    }

    /// Opens a volume using a file-like object.
    fn open_file_object(self, file_object: File, mode: &str) {
        unimplemented!();
    }

    /// Signals the volume to abort the current activity.
    fn signal_abort(self) {
        unimplemented!();
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        unsafe {
            libfsntfs_volume_free(&mut self.volume as *mut _, ptr::null_mut());
        }
    }
}
