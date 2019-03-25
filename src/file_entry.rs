use chrono::prelude::*;

use std::convert::TryFrom;
use crate::error::Error;
use crate::ffi::AsTypeRef;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};
use crate::libfsntfs::{libfsntfs_attribute_t, libfsntfs_data_stream_t, off64_t, size64_t};
use std::ffi::c_void;
use std::os::raw::c_int;
use std::{mem, ptr};

#[repr(C)]
pub struct __FileEntry(isize);

pub type FileEntryRef = *mut __FileEntry;

declare_ffi_type!(FileEntry, FileEntryRef);
impl_ffi_type!(FileEntry, FileEntryRef);

extern "C" {
    #[link_name = "\u{1}_libfsntfs_file_entry_free"]
    pub fn libfsntfs_file_entry_free(
        file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_is_empty"]
    pub fn libfsntfs_file_entry_is_empty(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_is_allocated"]
    pub fn libfsntfs_file_entry_is_allocated(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_file_reference"]
    pub fn libfsntfs_file_entry_get_file_reference(
        file_entry: FileEntryRef,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_base_record_file_reference"]
    pub fn libfsntfs_file_entry_get_base_record_file_reference(
        file_entry: FileEntryRef,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_parent_file_reference"]
    pub fn libfsntfs_file_entry_get_parent_file_reference(
        file_entry: FileEntryRef,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_parent_file_reference_by_attribute_index"]
    pub fn libfsntfs_file_entry_get_parent_file_reference_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_journal_sequence_number"]
    pub fn libfsntfs_file_entry_get_journal_sequence_number(
        file_entry: FileEntryRef,
        journal_sequence_number: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_creation_time"]
    pub fn libfsntfs_file_entry_get_creation_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_modification_time"]
    pub fn libfsntfs_file_entry_get_modification_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_access_time"]
    pub fn libfsntfs_file_entry_get_access_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_entry_modification_time"]
    pub fn libfsntfs_file_entry_get_entry_modification_time(
        file_entry: FileEntryRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_file_attribute_flags"]
    pub fn libfsntfs_file_entry_get_file_attribute_flags(
        file_entry: FileEntryRef,
        file_attribute_flags: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_name_size"]
    pub fn libfsntfs_file_entry_get_utf8_name_size(
        file_entry: FileEntryRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_name"]
    pub fn libfsntfs_file_entry_get_utf8_name(
        file_entry: FileEntryRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_name_size"]
    pub fn libfsntfs_file_entry_get_utf16_name_size(
        file_entry: FileEntryRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_name"]
    pub fn libfsntfs_file_entry_get_utf16_name(
        file_entry: FileEntryRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_name_attribute_index"]
    pub fn libfsntfs_file_entry_get_name_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_name_size_by_attribute_index"]
    pub fn libfsntfs_file_entry_get_utf8_name_size_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_name_by_attribute_index"]
    pub fn libfsntfs_file_entry_get_utf8_name_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_name_size_by_attribute_index"]
    pub fn libfsntfs_file_entry_get_utf16_name_size_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_name_by_attribute_index"]
    pub fn libfsntfs_file_entry_get_utf16_name_by_attribute_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_reparse_point_substitute_name_size"]
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_substitute_name_size(
        file_entry: FileEntryRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_reparse_point_substitute_name"]
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_substitute_name(
        file_entry: FileEntryRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_reparse_point_substitute_name_size"]
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_substitute_name_size(
        file_entry: FileEntryRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_reparse_point_substitute_name"]
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_substitute_name(
        file_entry: FileEntryRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_reparse_point_print_name_size"]
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_print_name_size(
        file_entry: FileEntryRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf8_reparse_point_print_name"]
    pub fn libfsntfs_file_entry_get_utf8_reparse_point_print_name(
        file_entry: FileEntryRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_reparse_point_print_name_size"]
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_print_name_size(
        file_entry: FileEntryRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_utf16_reparse_point_print_name"]
    pub fn libfsntfs_file_entry_get_utf16_reparse_point_print_name(
        file_entry: FileEntryRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_security_descriptor_size"]
    pub fn libfsntfs_file_entry_get_security_descriptor_size(
        file_entry: FileEntryRef,
        data_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_security_descriptor"]
    pub fn libfsntfs_file_entry_get_security_descriptor(
        file_entry: FileEntryRef,
        data: *mut u8,
        data_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_number_of_attributes"]
    pub fn libfsntfs_file_entry_get_number_of_attributes(
        file_entry: FileEntryRef,
        number_of_attributes: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_attribute_by_index"]
    pub fn libfsntfs_file_entry_get_attribute_by_index(
        file_entry: FileEntryRef,
        attribute_index: c_int,
        attribute: *mut *mut libfsntfs_attribute_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_has_directory_entries_index"]
    pub fn libfsntfs_file_entry_has_directory_entries_index(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_has_default_data_stream"]
    pub fn libfsntfs_file_entry_has_default_data_stream(
        file_entry: FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_number_of_alternate_data_streams"]
    pub fn libfsntfs_file_entry_get_number_of_alternate_data_streams(
        file_entry: FileEntryRef,
        number_of_alternate_data_streams: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_alternate_data_stream_by_index"]
    pub fn libfsntfs_file_entry_get_alternate_data_stream_by_index(
        file_entry: FileEntryRef,
        alternate_data_stream_index: c_int,
        alternate_data_stream: *mut *mut libfsntfs_data_stream_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_has_alternate_data_stream_by_utf8_name"]
    pub fn libfsntfs_file_entry_has_alternate_data_stream_by_utf8_name(
        file_entry: FileEntryRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_has_alternate_data_stream_by_utf16_name"]
    pub fn libfsntfs_file_entry_has_alternate_data_stream_by_utf16_name(
        file_entry: FileEntryRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_alternate_data_stream_by_utf8_name"]
    pub fn libfsntfs_file_entry_get_alternate_data_stream_by_utf8_name(
        file_entry: FileEntryRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        alternate_data_stream: *mut *mut libfsntfs_data_stream_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_alternate_data_stream_by_utf16_name"]
    pub fn libfsntfs_file_entry_get_alternate_data_stream_by_utf16_name(
        file_entry: FileEntryRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        alternate_data_stream: *mut *mut libfsntfs_data_stream_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_number_of_sub_file_entries"]
    pub fn libfsntfs_file_entry_get_number_of_sub_file_entries(
        file_entry: FileEntryRef,
        number_of_sub_file_entries: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_sub_file_entry_by_index"]
    pub fn libfsntfs_file_entry_get_sub_file_entry_by_index(
        file_entry: FileEntryRef,
        sub_file_entry_index: c_int,
        sub_file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_sub_file_entry_by_utf8_name"]
    pub fn libfsntfs_file_entry_get_sub_file_entry_by_utf8_name(
        file_entry: FileEntryRef,
        utf8_string: *const u8,
        utf8_string_length: usize,
        sub_file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_sub_file_entry_by_utf16_name"]
    pub fn libfsntfs_file_entry_get_sub_file_entry_by_utf16_name(
        file_entry: FileEntryRef,
        utf16_string: *const u16,
        utf16_string_length: usize,
        sub_file_entry: *mut FileEntryRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_read_buffer"]
    pub fn libfsntfs_file_entry_read_buffer(
        file_entry: FileEntryRef,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> isize;
    #[link_name = "\u{1}_libfsntfs_file_entry_read_buffer_at_offset"]
    pub fn libfsntfs_file_entry_read_buffer_at_offset(
        file_entry: FileEntryRef,
        buffer: *mut ::std::os::raw::c_void,
        buffer_size: usize,
        offset: off64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> isize;
    #[link_name = "\u{1}_libfsntfs_file_entry_seek_offset"]
    pub fn libfsntfs_file_entry_seek_offset(
        file_entry: FileEntryRef,
        offset: off64_t,
        whence: c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> off64_t;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_offset"]
    pub fn libfsntfs_file_entry_get_offset(
        file_entry: FileEntryRef,
        offset: *mut off64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_size"]
    pub fn libfsntfs_file_entry_get_size(
        file_entry: FileEntryRef,
        size: *mut size64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_number_of_extents"]
    pub fn libfsntfs_file_entry_get_number_of_extents(
        file_entry: FileEntryRef,
        number_of_extents: *mut c_int,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    #[link_name = "\u{1}_libfsntfs_file_entry_get_extent_by_index"]
    pub fn libfsntfs_file_entry_get_extent_by_index(
        file_entry: FileEntryRef,
        extent_index: c_int,
        extent_offset: *mut off64_t,
        extent_size: *mut size64_t,
        extent_flags: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
}

impl Drop for FileEntry {
    fn drop(&mut self) {
        unsafe { libfsntfs_file_entry_free(&mut self.as_type_ref() as *mut _, ptr::null_mut()) };
    }
}

impl FileEntry {
    /// Returns the access date and time.
    pub fn get_access_time(&self) -> Option<DateTime<Utc>> {
        unimplemented!();
    }

    pub fn get_access_time_as_integer(&self) {
        unimplemented!();
    }

    pub fn get_size(&mut self) -> Result<u64, Error> {
        let mut size = 0;
        let mut error = ptr::null_mut();

        unsafe {
            libfsntfs_file_entry_get_size(
                self.as_type_ref(),
                &mut size as *mut _,
                &mut error,
            );
        }

        if !error.is_null() {
            Ok(size)
        } else {
            Err(Error::try_from(error)?)
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

    pub fn get_attribute(&self, attribute_index: isize) {
        unimplemented!();
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

    pub fn get_name(&self) {
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

    pub fn get_number_of_attributes(&self) {
        unimplemented!();
    }

    pub fn get_number_of_extents(&self) {
        unimplemented!();
    }

    pub fn get_number_of_sub_file_entries(&self) {
        unimplemented!();
    }

    pub fn get_offset(&self) {
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
    pub fn is_allocated(&self) {
        unimplemented!();
    }

    pub fn is_empty(&self) {
        unimplemented!();
    }

    pub fn read(&self, size: isize) {
        unimplemented!();
    }

    pub fn read_buffer(&self, size: isize) {
        unimplemented!();
    }
    pub fn read_buffer_at_offset(&self, size: isize, offset: isize) {
        unimplemented!();
    }

    pub fn seek(&self, offset: isize, whence: isize) {
        unimplemented!();
    }
    pub fn seek_offset(&self, offset: isize, whence: isize) {
        unimplemented!();
    }
    pub fn tell(&self) {
        unimplemented!();
    }
}
