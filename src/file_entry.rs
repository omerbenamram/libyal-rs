use chrono::prelude::*;

use crate::attribute::{Attribute, AttributeRef};
use crate::error::Error;
use crate::ffi::AsTypeRef;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};
use crate::libfsntfs::{
    libfsntfs_attribute_t, libfsntfs_data_stream_t, off64_t, size64_t, SEEK_CUR, SEEK_END, SEEK_SET,
};
use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::fs::read;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::option::Iter;
use std::os::raw::c_int;
use std::{fmt, io, mem, ptr};

#[repr(C)]
pub struct __FileEntry(isize);

pub type FileEntryRef = *mut __FileEntry;

declare_ffi_type!(FileEntry, FileEntryRef);
impl_ffi_type!(FileEntry, FileEntryRef);
impl_ffi_dtor!(FileEntry, libfsntfs_file_entry_free);

impl Debug for FileEntry {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("FileEntry")
            .field("Name", &self.get_name().unwrap_or("".to_string()))
            .finish()
    }
}

extern "C" {
    pub fn libfsntfs_file_entry_free(
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_is_empty(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_is_allocated(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_file_reference(
        file_entry: FileEntryRef,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_base_record_file_reference(
        file_entry: FileEntryRef,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_parent_file_reference(
        file_entry: FileEntryRef,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_parent_file_reference_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_journal_sequence_number(
        file_entry: FileEntryRef,
        journal_sequence_number: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_creation_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_modification_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_access_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_entry_modification_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_file_attribute_flags(
        file_entry: FileEntryRef,
        file_attribute_flags: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_name_size(
        file_entry: FileEntryRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_name(
        file_entry: FileEntryRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_name_size(
        file_entry: FileEntryRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_name(
        file_entry: FileEntryRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_name_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_name_size_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_name_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_name_size_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_name_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_substitute_name_size(
        file_entry: FileEntryRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_substitute_name(
        file_entry: FileEntryRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_substitute_name_size(
        file_entry: FileEntryRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_substitute_name(
        file_entry: FileEntryRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_print_name_size(
        file_entry: FileEntryRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_print_name(
        file_entry: FileEntryRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_print_name_size(
        file_entry: FileEntryRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_print_name(
        file_entry: FileEntryRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_security_descriptor_size(
        file_entry: FileEntryRef,
        data_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_security_descriptor(
        file_entry: FileEntryRef,
        data: *mut u8,
        data_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_number_of_attributes(
        file_entry: FileEntryRef,
        number_of_attributes: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_attribute_by_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        attribute: *mut AttributeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_has_directory_entries_index(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_has_default_data_stream(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_number_of_alternate_data_streams(
        file_entry: FileEntryRef,
        number_of_alternate_data_streams: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_alternate_data_stream_by_index(
        file_entry: FileEntryRef,
        alternate_data_stream_index: c_int,
        alternate_data_stream: *mut *mut libfsntfs_data_stream_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_has_alternate_data_stream_by_utf8_name(
        file_entry: FileEntryRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_has_alternate_data_stream_by_utf16_name(
        file_entry: FileEntryRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_alternate_data_stream_by_utf8_name(
        file_entry: FileEntryRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        alternate_data_stream: *mut *mut libfsntfs_data_stream_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_alternate_data_stream_by_utf16_name(
        file_entry: FileEntryRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        alternate_data_stream: *mut *mut libfsntfs_data_stream_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_number_of_sub_file_entries(
        file_entry: FileEntryRef,
        number_of_sub_file_entries: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_sub_file_entry_by_index(
        file_entry: FileEntryRef,
        sub_file_entry_index: c_int,
        sub_file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_sub_file_entry_by_utf8_name(
        file_entry: FileEntryRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        sub_file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_sub_file_entry_by_utf16_name(
        file_entry: FileEntryRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        sub_file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_read_buffer(
        file_entry: FileEntryRef,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> isize;
    pub fn libfsntfs_file_entry_read_buffer_at_offset(
        file_entry: FileEntryRef,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        offset: off64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> isize;
    pub fn libfsntfs_file_entry_seek_offset(
        file_entry: FileEntryRef,
        offset: off64_t,
        whence: c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> off64_t;
    pub fn libfsntfs_file_entry_get_offset(
        file_entry: FileEntryRef,
        offset: *mut off64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_size(
        file_entry: FileEntryRef,
        size: *mut size64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_number_of_extents(
        file_entry: FileEntryRef,
        number_of_extents: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_entry_get_extent_by_index(
        file_entry: FileEntryRef,
        extent_index: c_int,
        extent_offset: *mut off64_t,
        extent_size: *mut size64_t,
        extent_flags: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
}

pub struct IterAttributes<'a> {
    handle: &'a FileEntry,
    num_attributes: u32,
    idx: u32,
}

impl<'a> Iterator for IterAttributes<'a> {
    type Item = Result<Attribute, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.num_attributes {
            let attr = self.handle.get_attribute_by_index(self.idx as i32);
            self.idx += 1;

            return Some(attr);
        }

        None
    }
}

impl Read for FileEntry {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut error = ptr::null_mut();
        let read_count = unsafe {
            libfsntfs_file_entry_read_buffer(
                self.as_type_ref(),
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                &mut error,
            )
        };

        if read_count <= -1 {
            let ffi_err = Error::try_from(error);

            let io_err = match ffi_err {
                Ok(e) => io::Error::new(io::ErrorKind::Other, format!("{}", e)),
                Err(e) => io::Error::new(
                    io::ErrorKind::Other,
                    format!("error while getting error information"),
                ),
            };

            Err(io_err)
        } else {
            Ok(read_count as usize)
        }
    }
}

impl Seek for FileEntry {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, io::Error> {
        let mut error = ptr::null_mut();

        let seek_pos = match pos {
            SeekFrom::Start(offset) => unsafe {
                libfsntfs_file_entry_seek_offset(
                    self.as_type_ref(),
                    offset as i64,
                    SEEK_SET as i32,
                    &mut error,
                )
            },
            SeekFrom::End(offset) => unsafe {
                libfsntfs_file_entry_seek_offset(
                    self.as_type_ref(),
                    offset as i64,
                    SEEK_END as i32,
                    &mut error,
                )
            },
            SeekFrom::Current(offset) => unsafe {
                libfsntfs_file_entry_seek_offset(
                    self.as_type_ref(),
                    offset as i64,
                    SEEK_CUR as i32,
                    &mut error,
                )
            },
        };

        if seek_pos <= -1 {
            let ffi_err = Error::try_from(error);

            let io_err = match ffi_err {
                Ok(e) => io::Error::new(io::ErrorKind::Other, format!("{}", e)),
                Err(e) => io::Error::new(
                    io::ErrorKind::Other,
                    format!("error while getting error information"),
                ),
            };

            Err(io_err)
        } else {
            Ok(seek_pos as u64)
        }
    }
}

impl FileEntry {
    /// Returns the access date and time.
    pub fn get_access_time(&self) -> Option<DateTime<Utc>> {
        unimplemented!();
    }

    pub fn get_size(&self) -> Result<u64, Error> {
        let mut size = 0;
        let mut error = ptr::null_mut();

        if unsafe { libfsntfs_file_entry_get_size(self.as_type_ref(), &mut size, &mut error) } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(size)
        }
    }

    /// Retrieves a specific alternate data stream.
    pub fn get_alternate_data_stream(&self, alternate_data_stream_index: isize) {
        unimplemented!();
    }

    /// Retrieves an alternate data stream specified by the name.
    pub fn get_alternate_data_stream_by_name(&self, name: isize) {
        unimplemented!();
    }

    pub fn iter_attributes(&self) -> Result<IterAttributes, Error> {
        let number_of_attributes = self.get_number_of_attributes()? as u32;

        Ok(IterAttributes {
            handle: self,
            num_attributes: number_of_attributes,
            idx: 0,
        })
    }

    pub fn get_number_of_attributes(&self) -> Result<c_int, Error> {
        let mut num_attributes = 0_i32;
        let mut error = ptr::null_mut();

        if unsafe {
            libfsntfs_file_entry_get_number_of_attributes(
                self.as_type_ref(),
                &mut num_attributes,
                &mut error,
            )
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(num_attributes)
        }
    }

    pub fn get_attribute_by_index(&self, attribute_index: i32) -> Result<Attribute, Error> {
        let mut attribute = ptr::null_mut();
        let mut error = ptr::null_mut();

        if unsafe {
            libfsntfs_file_entry_get_attribute_by_index(
                self.as_type_ref(),
                attribute_index,
                &mut attribute,
                &mut error,
            )
        } != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(Attribute::wrap_ptr(attribute))
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        get_sized_utf8_string!(
            self,
            libfsntfs_file_entry_get_utf8_name_size,
            libfsntfs_file_entry_get_utf8_name
        )
    }

    pub fn get_base_record_file_reference(&self) {
        unimplemented!();
    }

    pub fn get_creation_time(&self) {
        unimplemented!();
    }

    pub fn get_creation_time_as_integer(&self) {
        unimplemented!();
    }

    pub fn get_entry_modification_time(&self) {
        unimplemented!();
    }

    pub fn get_entry_modification_time_as_integer(&self) {
        unimplemented!();
    }

    pub fn get_extent(&self, extent_index: isize) {
        unimplemented!();
    }

    pub fn get_file_attribute_flags(&self) {
        unimplemented!();
    }

    pub fn get_file_reference(&self) {
        unimplemented!();
    }

    pub fn get_journal_sequence_number(&self) {
        unimplemented!();
    }

    pub fn get_modification_time(&self) {
        unimplemented!();
    }

    pub fn get_modification_time_as_integer(&self) {
        unimplemented!();
    }

    pub fn get_name_attribute_index(&self) {
        unimplemented!();
    }

    pub fn get_name_by_attribute_index(&self, attribute_index: isize) {
        unimplemented!();
    }

    pub fn get_number_of_alternate_data_streams(&self) {
        unimplemented!();
    }

    pub fn get_number_of_extents(&self) {
        unimplemented!();
    }

    pub fn get_number_of_sub_file_entries(&self) {
        unimplemented!();
    }

    pub fn get_parent_file_reference(&self) {
        unimplemented!();
    }

    pub fn get_parent_file_reference_by_attribute_index(&self, attribute_index: isize) {
        unimplemented!();
    }

    pub fn get_reparse_point_print_name(&self) {
        unimplemented!();
    }

    pub fn get_reparse_point_substitute_name(&self) {
        unimplemented!();
    }

    pub fn get_security_descriptor_data(&self) {
        unimplemented!();
    }

    pub fn get_sub_file_entry(&self, sub_file_entry_index: isize) {
        unimplemented!();
    }

    pub fn has_alternate_data_stream_by_name(&self, name: isize) {
        unimplemented!();
    }

    pub fn has_default_data_stream(&self) {
        unimplemented!();
    }

    pub fn has_directory_entries_index(&self) {
        unimplemented!();
    }

    pub fn is_empty(&self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::*;
    use log::{info, trace};
    use std::path::PathBuf;

    #[test]
    fn test_iter_attributes() {
        let mut volume = sample_volume().unwrap();
        let mut file_attribute = volume.get_file_entry_by_mft_idx(0).unwrap();

        for attribute in file_attribute
            .iter_attributes()
            .unwrap()
            .map(|a| a.unwrap())
        {
            println!("{:?}", attribute.get_name().unwrap());
            println!("{:?}", attribute.get_type().unwrap());
        }
    }

    #[test]
    fn test_read() {
        let mut entry = file_entry().unwrap();

        let mut buf = [0; 4096];
        entry.read(&mut buf);

        assert!(!buf.is_empty())
    }
}
