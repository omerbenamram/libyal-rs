use log::error;
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
use std::ffi::{c_void, CStr, CString};
use std::fs::File;
use std::mem;
use std::os::raw::c_int;
use std::path::{Iter, Path, PathBuf};
use std::ptr;

#[repr(C)]
pub struct __Volume(isize);

pub type VolumeRef = *mut __Volume;

declare_ffi_type!(Volume, VolumeRef);
impl_ffi_type!(Volume, VolumeRef);

extern "C" {
    /// Creates a volume
    /// Make sure the value volume is referencing, is set to NULL
    /// Returns 1 if successful or -1 on error
    pub fn libfsntfs_volume_initialize(
        volume: *mut VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    /// Frees a volume
    /// Returns 1 if successful or -1 on error
    pub fn libfsntfs_volume_free(volume: *mut VolumeRef, error: *mut LibfsntfsErrorRef) -> c_int;
    pub fn libfsntfs_volume_signal_abort(volume: VolumeRef, error: *mut LibfsntfsErrorRef)
        -> c_int;
    pub fn libfsntfs_volume_open(
        volume: VolumeRef,
        filename: *const ::std::os::raw::c_char,
        access_flags: c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_close(volume: VolumeRef, error: *mut LibfsntfsErrorRef) -> c_int;
    pub fn libfsntfs_volume_has_bitlocker_drive_encryption(
        volume: VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_has_volume_shadow_snapshots(
        volume: VolumeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_cluster_block_size(
        volume: VolumeRef,
        cluster_block_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_mft_entry_size(
        volume: VolumeRef,
        mft_entry_size: *mut size32_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_index_entry_size(
        volume: VolumeRef,
        index_entry_size: *mut size32_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_utf8_name_size(
        volume: VolumeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_utf8_name(
        volume: VolumeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_utf16_name_size(
        volume: VolumeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_utf16_name(
        volume: VolumeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_version(
        volume: VolumeRef,
        major_version: *mut u8,
        minor_version: *mut u8,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_serial_number(
        volume: VolumeRef,
        serial_number: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_number_of_file_entries(
        volume: VolumeRef,
        number_of_file_entries: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_file_entry_by_index(
        volume: VolumeRef,
        mft_entry_index: u64,
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_root_directory(
        volume: VolumeRef,
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_file_entry_by_utf8_path(
        volume: VolumeRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_get_file_entry_by_utf16_path(
        volume: VolumeRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
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
pub type SerialNumber = u64;

pub struct IterFileEntries<'a> {
    handle: &'a Volume,
    number_of_file_entries: usize,
    idx: usize,
}

impl<'a> Iterator for IterFileEntries<'a> {
    type Item = Result<FileEntry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.number_of_file_entries {
            let entry = self
                .handle
                .get_file_entry_by_mft_idx(self.idx as MftEntryIndex);
            self.idx += 1;

            return Some(entry);
        }

        None
    }
}

impl Volume {
    /// Opens a volume by filename.
    pub fn open(filename: impl AsRef<str>, mode: AccessMode) -> Result<Self, Error> {
        let mut handle: VolumeRef = ptr::null_mut();

        let c_string = CString::new(filename.as_ref()).map_err(Error::StringContainsNul)?;

        let mut init_error: LibfsntfsErrorRef = ptr::null_mut();

        let retcode =
            unsafe { libfsntfs_volume_initialize(&mut handle as _, &mut init_error as _) };

        if retcode != 1 {
            return Err(Error::try_from(init_error)?);
        }

        let volume = Volume::wrap_ptr(handle);

        let mut error = ptr::null_mut();

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

    pub fn iter_entries(&self) -> Result<IterFileEntries, Error> {
        Ok(IterFileEntries {
            handle: self,
            number_of_file_entries: self.get_number_of_file_entries()?,
            idx: 0,
        })
    }

    /// Retrieves the volume serial number.
    pub fn get_serial_number(&self) -> Result<SerialNumber, Error> {
        let mut serial_number = 0_u64;
        let mut error = ptr::null_mut();

        if unsafe {
            libfsntfs_volume_get_serial_number(self.as_type_ref(), &mut serial_number, &mut error)
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(serial_number)
        }
    }

    /// Retrieves a file entry specified by the path.
    pub fn get_file_entry_by_path(&self, path: impl AsRef<Path>) -> Result<FileEntry, Error> {
        let mut file_entry = ptr::null_mut();
        let mut error = ptr::null_mut();

        let path_as_str = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::Other("String is invalid UTF-8".to_owned()))?;

        if unsafe {
            libfsntfs_volume_get_file_entry_by_utf8_path(
                self.as_type_ref(),
                path_as_str.as_ptr(),
                path_as_str.len(),
                &mut file_entry,
                &mut error,
            )
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(FileEntry::wrap_ptr(file_entry))
        }
    }

    /// Retrieves a specific file entry.
    pub fn get_file_entry_by_mft_idx(&self, idx: MftEntryIndex) -> Result<FileEntry, Error> {
        let mut file_entry = ptr::null_mut();
        let mut error = ptr::null_mut();

        if unsafe {
            libfsntfs_volume_get_file_entry_by_index(
                self.as_type_ref(),
                idx,
                &mut file_entry,
                &mut error,
            )
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(FileEntry::wrap_ptr(file_entry))
        }
    }

    /// Retrieves the name.
    pub fn get_name(&self) -> Result<String, Error> {
        get_sized_utf8_string!(
            self,
            libfsntfs_volume_get_utf8_name_size,
            libfsntfs_volume_get_utf8_name
        )
    }

    /// Closes a volume.
    fn close(&self) {
        unimplemented!();
    }

    /// Retrieves the root directory.
    fn get_root_directory(&self) -> Result<FileEntry, Error> {
        let mut file_entry = ptr::null_mut();
        let mut error = ptr::null_mut();

        if unsafe {
            libfsntfs_volume_get_root_directory(self.as_type_ref(), &mut file_entry, &mut error)
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(FileEntry::wrap_ptr(file_entry))
        }
    }

    /// Retrieves the number of file entries.
    fn get_number_of_file_entries(&self) -> Result<usize, Error> {
        let mut number_of_file_entries = 0;
        let mut error = ptr::null_mut();

        if unsafe {
            libfsntfs_volume_get_number_of_file_entries(
                self.as_type_ref(),
                &mut number_of_file_entries,
                &mut error,
            )
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(number_of_file_entries as usize)
        }
    }

    /// Retrieves the USN change journal.
    fn get_usn_change_journal(&self) {
        unimplemented!();
    }

    /// Signals the volume to abort the current activity.
    fn signal_abort(&self) {
        unimplemented!();
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        let mut error = ptr::null_mut();

        if unsafe { libfsntfs_volume_close(self.as_type_ref(), &mut error) } != 1 {
            error!("`libfsntfs_volume_close` failed!");
        }

        let mut error = ptr::null_mut();
        if unsafe { libfsntfs_volume_free(&mut self.as_type_ref(), &mut error) } != 1 {
            panic!("`libfsntfs_volume_free` failed!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::*;
    use log::{info, trace};
    use std::path::PathBuf;

    #[test]
    fn test_opens_volume_works() {
        assert!(sample_volume().is_ok());
    }

    #[test]
    fn test_get_volume_name_works() {
        let volume_name_result = sample_volume().unwrap().get_name();
        assert!(
            volume_name_result.is_ok(),
            "FFI call to get_volume_name failed"
        );
        assert_eq!(volume_name_result.unwrap(), "KW-SRCH-1")
    }

    #[test]
    fn test_get_serial_number() {
        let volume_name_result = sample_volume().unwrap().get_serial_number();
        assert!(
            volume_name_result.is_ok(),
            "FFI call to get_volume_name failed"
        );
        assert_eq!(volume_name_result.unwrap(), 13425491701870188067)
    }

    #[test]
    fn test_iter_entries() {
        let volume = sample_volume().unwrap();

        for result in volume.iter_entries().unwrap() {
            let entry = result.unwrap();
            println!("{:?}", entry);
        }
    }
}
