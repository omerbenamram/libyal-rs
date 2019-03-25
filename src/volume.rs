use crate::error::{Error};
use crate::ffi::AsFFIPtr;
use crate::file_entry::FileEntry;
use std::ffi::{CString, c_void};
use std::fs::File;
use std::os::raw::c_int;
use std::path::{Path, PathBuf};
use std::ptr;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};
use crate::libfsntfs::{libfsntfs_file_entry_t, LIBFSNTFS_ACCESS_FLAGS, LIBFSNTFS_ACCESS_FLAGS_LIBFSNTFS_ACCESS_FLAG_READ, size32_t, LIBFSNTFS_ACCESS_FLAGS_LIBFSNTFS_ACCESS_FLAG_WRITE};

#[repr(C)]
struct __Volume(c_void);

pub type VolumeRef = *mut __Volume;

declare_ffi_type!(Volume, VolumeRef);
impl_ffi_type!(Volume, VolumeRef);

extern "C" {
    #[link_name = "\u{1}_libfsntfs_volume_initialize"]
    pub fn libfsntfs_volume_initialize(
        volume: *mut VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_free"]
    pub fn libfsntfs_volume_free(
        volume: *mut VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_signal_abort"]
    pub fn libfsntfs_volume_signal_abort(
        volume: VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_open"]
    pub fn libfsntfs_volume_open(
        volume: VolumeRef,
        filename: *const ::std::os::raw::c_char,
        access_flags: ::std::os::raw::c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_close"]
    pub fn libfsntfs_volume_close(
        volume: VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_has_bitlocker_drive_encryption"]
    pub fn libfsntfs_volume_has_bitlocker_drive_encryption(
        volume: VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_has_volume_shadow_snapshots"]
    pub fn libfsntfs_volume_has_volume_shadow_snapshots(
        volume: VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_cluster_block_size"]
    pub fn libfsntfs_volume_get_cluster_block_size(
        volume: VolumeRef,
        cluster_block_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_mft_entry_size"]
    pub fn libfsntfs_volume_get_mft_entry_size(
        volume: VolumeRef,
        mft_entry_size: *mut size32_t,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_index_entry_size"]
    pub fn libfsntfs_volume_get_index_entry_size(
        volume: VolumeRef,
        index_entry_size: *mut size32_t,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_utf8_name_size"]
    pub fn libfsntfs_volume_get_utf8_name_size(
        volume: VolumeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_utf8_name"]
    pub fn libfsntfs_volume_get_utf8_name(
        volume: VolumeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_utf16_name_size"]
    pub fn libfsntfs_volume_get_utf16_name_size(
        volume: VolumeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_utf16_name"]
    pub fn libfsntfs_volume_get_utf16_name(
        volume: VolumeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_version"]
    pub fn libfsntfs_volume_get_version(
        volume: VolumeRef,
        major_version: *mut u8,
        minor_version: *mut u8,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_serial_number"]
    pub fn libfsntfs_volume_get_serial_number(
        volume: VolumeRef,
        serial_number: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_number_of_file_entries"]
    pub fn libfsntfs_volume_get_number_of_file_entries(
        volume: VolumeRef,
        number_of_file_entries: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_file_entry_by_index"]
    pub fn libfsntfs_volume_get_file_entry_by_index(
        volume: VolumeRef,
        mft_entry_index: u64,
        file_entry: *mut *mut libfsntfs_file_entry_t,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_root_directory"]
    pub fn libfsntfs_volume_get_root_directory(
        volume: VolumeRef,
        file_entry: *mut *mut libfsntfs_file_entry_t,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_file_entry_by_utf8_path"]
    pub fn libfsntfs_volume_get_file_entry_by_utf8_path(
        volume: VolumeRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        file_entry: *mut *mut libfsntfs_file_entry_t,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_file_entry_by_utf16_path"]
    pub fn libfsntfs_volume_get_file_entry_by_utf16_path(
        volume: VolumeRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        file_entry: *mut *mut libfsntfs_file_entry_t,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
}

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
    pub fn open(filename: impl AsRef<str>, mode: &AccessMode) -> Result<Self, Error> {
        let mut handle: Volume = ptr::null_mut();
        let mut c_string = CString::new(filename.as_ref()).expect("Should not contain NUL");
        let mut error: LibfsntfsError = ptr::null_mut();

        unsafe {
            libfsntfs_volume_open(
                handle.as_ffi_ptr(),
                c_string.as_ptr(),
                mode.as_flag() as c_int,
                error.as_ffi_ptr(),
            );
        }

        if error.is_null() {
            Ok(handle)
        } else {
            Err(Error::ffi(error))
        }
    }

    /// Retrieves a file entry specified by the path.
    pub fn get_file_entry_by_path(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<FileEntry, Error> {
        let mut file_entry: FileEntry = ptr::null_mut();
        let mut error: LibfsntfsError = ptr::null_mut();

        let path_as_str = path.as_ref().to_str().expect("should be a valid UTF8");

        unsafe {
            libfsntfs_volume_get_file_entry_by_utf8_path(
                self.as_ffi_ptr(),
                path_as_str.as_ptr(),
                path_as_str.len(),
                file_entry.as_ffi_ptr(),
                error.as_ffi_ptr(),
            );
        }

        if error.is_null() {
            Ok(file_entry)
        } else {
            Err(Error::ffi(error))
        }
    }

    /// Retrieves a specific file entry.
    pub fn get_file_entry_by_mft_idx(
        &self,
        idx: MftEntryIndex,
    ) -> Result<FileEntry, Error> {
        let mut file_entry = ptr::null_mut();
        let mut error = ptr::null_mut();

        unsafe {
            libfsntfs_volume_get_file_entry_by_index(
                self.as_ffi_ptr(),
                idx,
                file_entry.as_ffi_ptr(),
                error,
            );
        }

        if error.is_null() {
            Ok(file_entry)
        } else {
            Err(Error::ffi(error))
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
