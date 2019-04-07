use crate::libbfio::*;
use libyal_rs_common::ffi::AsTypeRef;
use std::fs::File;
use std::os::raw::c_int;
use std::ptr;

#[repr(C)]
pub struct __Handle(isize);

pub type HandleRefMut = *mut __Handle;
pub type HandleRef = *const __Handle;

#[repr(C)]
pub struct Handle<'a>(HandleRefMut, &'a File);

impl<'a> AsTypeRef for Handle<'a> {
    type Ref = HandleRef;
    type RefMut = HandleRefMut;

    #[inline]
    fn as_type_ref(&self) -> Self::Ref {
        self.0 as *const _
    }

    #[inline]
    fn as_type_ref_mut(&mut self) -> Self::RefMut {
        self.0
    }

    #[inline]
    fn as_raw(&mut self) -> *mut Self::RefMut {
        &mut self.0 as *mut _
    }
}

//impl<'a> Handle<'a> {
//    pub fn wrap_ptr(volume: &'a Volume, ptr: FileEntryRefMut) -> Self {
//        Handle(ptr, volume)
//    }
//}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        use libyal_rs_common::ffi::AsTypeRef;
        use log::trace;

        let mut error = ptr::null_mut();

        trace!("Calling `{}`", module_path!());

        unsafe {
            libbfio_handle_free(&mut self.as_type_ref_mut() as *mut _, &mut error);
        }

        debug_assert!(error.is_null(), "`{}` failed!", module_path!());
    }
}

extern "C" {
    pub fn libbfio_handle_initialize(
        handle: *mut HandleRefMut,
        io_handle: *mut isize,
        free_io_handle: Option<
            unsafe extern "C" fn(
                io_handle: *mut HandleRefMut,
                error: *mut *mut libbfio_error_t,
            ) -> c_int,
        >,
        clone_io_handle: Option<
            unsafe extern "C" fn(
                destination_io_handle: *mut HandleRefMut,
                source_io_handle: *mut isize,
                error: *mut *mut libbfio_error_t,
            ) -> c_int,
        >,
        open: Option<
            unsafe extern "C" fn(
                io_handle: *mut isize,
                access_flags: c_int,
                error: *mut *mut libbfio_error_t,
            ) -> c_int,
        >,
        close: Option<
            unsafe extern "C" fn(io_handle: *mut isize, error: *mut *mut libbfio_error_t) -> c_int,
        >,
        read: Option<
            unsafe extern "C" fn(
                io_handle: *mut isize,
                buffer: *mut u8,
                size: usize,
                error: *mut *mut libbfio_error_t,
            ) -> isize,
        >,
        write: Option<
            unsafe extern "C" fn(
                io_handle: *mut isize,
                buffer: *const u8,
                size: usize,
                error: *mut *mut libbfio_error_t,
            ) -> isize,
        >,
        seek_offset: Option<
            unsafe extern "C" fn(
                io_handle: *mut isize,
                offset: off64_t,
                whence: c_int,
                error: *mut *mut libbfio_error_t,
            ) -> off64_t,
        >,
        exists: Option<
            unsafe extern "C" fn(io_handle: *mut isize, error: *mut *mut libbfio_error_t) -> c_int,
        >,
        is_open: Option<
            unsafe extern "C" fn(io_handle: *mut isize, error: *mut *mut libbfio_error_t) -> c_int,
        >,
        get_size: Option<
            unsafe extern "C" fn(
                io_handle: *mut isize,
                size: *mut size64_t,
                error: *mut *mut libbfio_error_t,
            ) -> c_int,
        >,
        flags: u8,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;

    pub fn libbfio_handle_free(
        handle: *mut HandleRefMut,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_clone(
        destination_handle: *mut HandleRefMut,
        source_handle: *mut libbfio_handle_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_open(
        handle: *mut libbfio_handle_t,
        access_flags: c_int,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_reopen(
        handle: *mut libbfio_handle_t,
        access_flags: c_int,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_close(
        handle: *mut libbfio_handle_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_read_buffer(
        handle: *mut libbfio_handle_t,
        buffer: *mut u8,
        size: usize,
        error: *mut *mut libbfio_error_t,
    ) -> isize;
    pub fn libbfio_handle_write_buffer(
        handle: *mut libbfio_handle_t,
        buffer: *const u8,
        size: usize,
        error: *mut *mut libbfio_error_t,
    ) -> isize;
    pub fn libbfio_handle_seek_offset(
        handle: *mut libbfio_handle_t,
        offset: off64_t,
        whence: c_int,
        error: *mut *mut libbfio_error_t,
    ) -> off64_t;
    pub fn libbfio_handle_exists(
        handle: *mut libbfio_handle_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_is_open(
        handle: *mut libbfio_handle_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_get_io_handle(
        handle: *mut libbfio_handle_t,
        io_handle: *mut HandleRefMut,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_get_access_flags(
        handle: *mut libbfio_handle_t,
        access_flags: *mut c_int,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_set_access_flags(
        handle: *mut libbfio_handle_t,
        access_flags: c_int,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_get_offset(
        handle: *mut libbfio_handle_t,
        offset: *mut off64_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_get_size(
        handle: *mut libbfio_handle_t,
        size: *mut size64_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_set_open_on_demand(
        handle: *mut libbfio_handle_t,
        open_on_demand: u8,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_set_track_offsets_read(
        handle: *mut libbfio_handle_t,
        track_offsets_read: u8,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_get_number_of_offsets_read(
        handle: *mut libbfio_handle_t,
        number_of_read_offsets: *mut c_int,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_get_offset_read(
        handle: *mut libbfio_handle_t,
        index: c_int,
        offset: *mut off64_t,
        size: *mut size64_t,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
}
