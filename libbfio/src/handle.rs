use std::fs::File;
use std::os::raw::c_int;
use std::ptr;

#[repr(C)]
pub struct __Handle(isize);

pub type HandleRefMut = *mut __Handle;
pub type HandleRef = *const __Handle;

#[repr(C)]
pub struct Handle<'a>(HandleRefMut, &'a File);

impl<'a> crate::ffi::AsTypeRef for Handle<'a> {
    type Ref = HandleRef;
    type RefMut = FileEntryRefMut;

    #[inline]
    fn as_type_ref(&self) -> Self::Ref {
        // https://users.rust-lang.org/t/is-it-ub-to-convert-t-to-mut-t/16238/4
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

impl<'a> Handle<'a> {
    pub fn wrap_ptr(volume: &'a Volume, ptr: FileEntryRefMut) -> Self {
        Handle(ptr, volume)
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        use crate::ffi::AsTypeRef;
        use log::trace;

        let mut error = ptr::null_mut();

        trace!("Calling `libfsntfs_file_entry_free`");

        unsafe {
            libfsntfs_file_entry_free(&mut self.as_type_ref_mut() as *mut _, &mut error);
        }

        debug_assert!(error.is_null(), "`libfsntfs_file_entry_free` failed!");
    }
}

type FreeFnPtr =
    unsafe extern "C" fn(_: *mut *mut intptr_t, _: *mut *mut libcerror_error_t) -> libc::c_int;

type CloneHandleFnPtr = unsafe extern "C" fn(
    _: *mut *mut intptr_t,
    _: *mut intptr_t,
    _: *mut *mut libcerror_error_t,
) -> libc::c_int;

type OpenHandleFnPtr = CloneHandleFnPtr;

type CloseHandleFnPtr =
    unsafe extern "C" fn(_: *mut intptr_t, _: *mut *mut libcerror_error_t) -> libc::c_int;

type ReadHandleFnPtr = unsafe extern "C" fn(
    file_handle: *mut intptr_t,
    buffer: *mut uint8_t,
    n_bytes: size_t,
    err: *mut *mut libcerror_error_t,
) -> ssize_t;

type WriteHandleFnPtr = unsafe extern "C" fn(
    _: *mut intptr_t,
    _: *const uint8_t,
    _: size_t,
    _: *mut *mut libcerror_error_t,
) -> ssize_t;

type SeekHandleFnPtr = unsafe extern "C" fn(
    _: *mut intptr_t,
    _: off64_t,
    _: libc::c_int,
    _: *mut *mut libcerror_error_t,
) -> off64_t;

type SelfFnPtr =
    unsafe extern "C" fn(_: *mut intptr_t, _: *mut *mut libcerror_error_t) -> libc::c_int;

type GetSizeFnPtr = unsafe extern "C" fn(
    _: *mut intptr_t,
    _: *mut size64_t,
    _: *mut *mut libcerror_error_t,
) -> libc::c_int;

extern "C" {
    pub fn libbfio_handle_initialize(
        handle: *mut FileEntryRefMutt,
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
        handle: *mut FileEntryRefMutt,
        error: *mut *mut libbfio_error_t,
    ) -> c_int;
    pub fn libbfio_handle_clone(
        destination_handle: *mut FileEntryRefMutt,
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
