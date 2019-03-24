use crate::error::LibfsntfsError;
use crate::ffi::AsFFIPtr;
use crate::file_entry::FileEntry;
use crate::libfsntfs::*;
use std::ffi::CString;
use std::fs::File;
use std::os::raw::c_int;
use std::path::{Path, PathBuf};
use std::ptr;

pub struct Volume {
    volume: isize,
}

impl_as_ffi_ptr!(isize, volume, Volume);

pub enum AccessMode {
    Read,
    Write,
}

impl AccessMode {
    fn as_flag(&self) -> LIBFSNTFS_ACCESS_FLAGS {
        match self {
            AccessMode::Read => LIBFSNTFS_ACCESS_FLAGS_LIBFSNTFS_ACCESS_FLAG_READ,
            AccessMode::Write => LIBFSNTFS_ACCESS_FLAGS_LIBFSNTFS_ACCESS_FLAG_WRITE,
        }
    }
}

pub type MftEntryIndex = u64;

impl Volume {
    /// Opens a volume by filename. will panic if filename contains a nul byte.
    pub fn open(filename: impl AsRef<str>, mode: &AccessMode) -> Result<Self, LibfsntfsError> {
        let mut handle = 0;
        let mut c_string = CString::new(filename.as_ref()).expect("Should not contain NUL");
        let mut error = LibfsntfsError::new();

        unsafe {
            libfsntfs_volume_open(
                &mut handle as *mut _,
                c_string.as_ptr(),
                mode.as_flag() as c_int,
                error.as_ffi_ptr(),
            );
        }

        if !error.is_error() {
            Ok(Volume { volume: handle })
        } else {
            Err(error)
        }
    }

    /// Retrieves a file entry specified by the path.
    pub fn get_file_entry_by_path(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<FileEntry, LibfsntfsError> {
        let mut file_entry = 0;
        let mut error = LibfsntfsError::new();

        let path_as_str = path.as_ref().to_str().expect("should be a valid UTF8");

        unsafe {
            libfsntfs_volume_get_file_entry_by_utf8_path(
                &mut self.volume.clone() as *mut _,
                path_as_str.as_ptr(),
                path_as_str.len(),
                file_entry.as_ffi_ptr(),
                error.as_ffi_ptr(),
            );
        }

        if error.is_error() {
            Ok(FileEntry { file_entry })
        } else {
            Err(error)
        }
    }

    /// Retrieves a specific file entry.
    pub fn get_file_entry_by_mft_idx(
        &self,
        idx: MftEntryIndex,
    ) -> Result<FileEntry, LibfsntfsError> {
        let mut file_entry: isize = 0;
        let mut error = LibfsntfsError::new();

        unsafe {
            libfsntfs_volume_get_file_entry_by_index(
                &mut self.volume.clone() as *mut _,
                idx,
                file_entry.as_ffi_ptr(),
                error.as_ffi_ptr(),
            );
        }

        if error.is_error() {
            Ok(FileEntry { file_entry })
        } else {
            Err(error)
        }
    }

    /// Closes a volume.
    fn close(&self) {
        unimplemented!();
    }

    /// Retrieves the name.
    fn get_name(&self) {
        unimplemented!();
    }

    /// Retrieves the number of file entries.
    fn get_number_of_file_entries(&self) {
        unimplemented!();
    }

    /// Retrieves the root directory.
    fn get_root_directory(&self) {
        unimplemented!();
    }

    /// Retrieves the USN change journal.
    fn get_usn_change_journal(&self) {
        unimplemented!();
    }

    /// Opens a volume using a file-like object.
    fn open_file_object(&self, file_object: File, mode: &str) {
        unimplemented!();
    }

    /// Signals the volume to abort the current activity.
    fn signal_abort(&self) {
        unimplemented!();
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        unsafe {
            libfsntfs_volume_free(self.as_ffi_ptr(), ptr::null_mut());
        }
    }
}
