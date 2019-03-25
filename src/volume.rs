use crate::error::Error;
use crate::ffi::AsTypeRef;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};
use crate::file_entry::{FileEntry, FileEntryRef};
use crate::libfsntfs::{
    libfsntfs_file_entry_t, size32_t, LIBFSNTFS_ACCESS_FLAGS,
    LIBFSNTFS_ACCESS_FLAGS_LIBFSNTFS_ACCESS_FLAG_READ,
    LIBFSNTFS_ACCESS_FLAGS_LIBFSNTFS_ACCESS_FLAG_WRITE,
};
use std::convert::TryFrom;
use std::ffi::{c_void, CString};
use std::fs::File;
use std::mem;
use std::os::raw::c_int;
use std::path::{Path, PathBuf};
use std::ptr;

#[repr(C)]
pub struct __Volume(isize);

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
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_root_directory"]
    pub fn libfsntfs_volume_get_root_directory(
        volume: VolumeRef,
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_file_entry_by_utf8_path"]
    pub fn libfsntfs_volume_get_file_entry_by_utf8_path(
        volume: VolumeRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_volume_get_file_entry_by_utf16_path"]
    pub fn libfsntfs_volume_get_file_entry_by_utf16_path(
        volume: VolumeRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        file_entry: *mut FileEntryRef,
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
        let mut handle: VolumeRef = ptr::null_mut();
        let mut c_string = CString::new(filename.as_ref()).expect("Should not contain NUL");
        let mut init_error: LibfsntfsErrorRef = ptr::null_mut();

        let retcode =
            unsafe { libfsntfs_volume_initialize(&mut handle as _, &mut init_error as _) };

        if retcode != 1 {
            println!("libfsntfs_volume_initialize returned: {}", retcode);
            return Err(Error::try_from(init_error)?);
        }

        let mut volume = Volume::wrap_ptr(handle);

        let mut error= ptr::null_mut();

        if unsafe {
            libfsntfs_volume_open(
                volume.as_type_ref(),
                c_string.as_ptr(),
                mode.as_flag() as c_int,
                &mut error as _,
            )
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(volume)
        }
    }

    /// Retrieves a file entry specified by the path.
    pub fn get_file_entry_by_path(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut file_entry = ptr::null_mut();
        let mut error = ptr::null_mut();

        let path_as_str = path.as_ref().to_str().expect("should be a valid UTF8");

        unsafe {
            libfsntfs_volume_get_file_entry_by_utf8_path(
                self.as_type_ref(),
                path_as_str.as_ptr(),
                path_as_str.len(),
                file_entry,
                &mut error,
            );
        }

        if error.is_null() {
            Ok(())
        } else {
            Err(Error::try_from(error)?)
        }
    }

    /// Retrieves a specific file entry.
    pub fn get_file_entry_by_mft_idx(&mut self, idx: MftEntryIndex) -> Result<(), ()> {
        let mut file_entry = ptr::null_mut();
        let mut error = ptr::null_mut();

        unsafe {
            libfsntfs_volume_get_file_entry_by_index(self.as_type_ref(), idx, file_entry, error);
        }

        if error.is_null() {
            Ok(())
        } else {
            Err(())
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
            libfsntfs_volume_free(&mut self.as_type_ref() as *mut _, ptr::null_mut());
        }
    }
}
