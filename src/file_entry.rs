use crate::libfsntfs::*;
use chrono::prelude::*;

use std::ptr;

pub struct FileEntry {
    file_entry: isize,
}

impl FileEntry {

    /// Returns the access date and time.
    pub fn get_access_time(self) -> Option<DateTime<Utc>> {
        unimplemented!();
    }

    pub fn get_access_time_as_integer(self) {
        unimplemented!();
    }

    /// Retrieves a specific alternate data stream.
    pub fn get_alternate_data_stream(self, alternate_data_stream_index: isize) {
        unimplemented!();
    }

    /// Retrieves an alternate data stream specified by the name.
    pub fn get_alternate_data_stream_by_name(self, name: isize) {
        unimplemented!();
    }

    pub fn get_attribute(self, attribute_index: isize) {
        unimplemented!();
    }

    pub fn get_base_record_file_reference(self) {
        unimplemented!();
    }

    pub fn get_creation_time(self) {
        unimplemented!();
    }

    pub fn get_creation_time_as_integer(self) {
        unimplemented!();
    }

    pub fn get_entry_modification_time(self) {
        unimplemented!();
    }

    pub fn get_entry_modification_time_as_integer(self) {
        unimplemented!();
    }

    pub fn get_extent(self, extent_index: isize) {
        unimplemented!();
    }

    pub fn get_file_attribute_flags(self) {
        unimplemented!();
    }

    pub fn get_file_reference(self) {
        unimplemented!();
    }

    pub fn get_journal_sequence_number(self) {
        unimplemented!();
    }

    pub fn get_modification_time(self) {
        unimplemented!();
    }

    pub fn get_modification_time_as_integer(self) {
        unimplemented!();
    }

    pub fn get_name(self) {
        unimplemented!();
    }

    pub fn get_name_attribute_index(self) {
        unimplemented!();
    }

    pub fn get_name_by_attribute_index(self, attribute_index: isize) {
        unimplemented!();
    }

    pub fn get_number_of_alternate_data_streams(self) {
        unimplemented!();
    }

    pub fn get_number_of_attributes(self) {
        unimplemented!();
    }

    pub fn get_number_of_extents(self) {
        unimplemented!();
    }

    pub fn get_number_of_sub_file_entries(self) {
        unimplemented!();
    }

    pub fn get_offset(self) {
        unimplemented!();
    }

    pub fn get_parent_file_reference(self) {
        unimplemented!();
    }

    pub fn get_parent_file_reference_by_attribute_index(self, attribute_index: isize) {
        unimplemented!();
    }

    pub fn get_reparse_point_print_name(self) {
        unimplemented!();
    }

    pub fn get_reparse_point_substitute_name(self) {
        unimplemented!();
    }

    pub fn get_security_descriptor_data(self) {
        unimplemented!();
    }

    pub fn get_size(self) {
        unimplemented!();
    }

    pub fn get_sub_file_entry(self, sub_file_entry_index: isize) {
        unimplemented!();
    }

    pub fn has_alternate_data_stream_by_name(self, name: isize) {
        unimplemented!();
    }

    pub fn has_default_data_stream(self) {
        unimplemented!();
    }

    pub fn has_directory_entries_index(self) {
        unimplemented!();
    }
    pub fn is_allocated(self) {
        unimplemented!();
    }

    pub fn is_empty(self) {
        unimplemented!();
    }

    pub fn read(self, size: isize) {
        unimplemented!();
    }

    pub fn read_buffer(self, size: isize) {
        unimplemented!();
    }
    pub fn read_buffer_at_offset(self, size: isize, offset: isize) {
        unimplemented!();
    }

    pub fn seek(self, offset: isize, whence: isize) {
        unimplemented!();
    }
    pub fn seek_offset(self, offset: isize, whence: isize) {
        unimplemented!();
    }
    pub fn tell(self) {
        unimplemented!();
    }
}

impl Drop for FileEntry {
    fn drop(&mut self) {
        unsafe { libfsntfs_file_entry_free(&mut self.file_entry as *mut _, ptr::null_mut()) };
    }
}
