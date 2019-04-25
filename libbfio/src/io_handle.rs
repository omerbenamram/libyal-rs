use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use libbfio_sys::{size64_t, SEEK_CUR, SEEK_END, SEEK_SET};
use log::trace;

use libcerror_sys::*;
use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::c_int;
use std::path::Path;
use std::{slice, io};

pub trait RwSeek: Read + Write + Seek {}
impl<T: Read + Write + Seek> RwSeek for T {}

pub struct IoHandle {
    inner: Box<dyn RwSeek>,
    is_open: bool,
    size: u64,
}

impl IoHandle {
    pub fn file(inner: File) -> io::Result<Self> {
        let metadata = inner.metadata()?;

        Ok(IoHandle {
            inner: Box::new(inner) as Box<dyn RwSeek>,
            is_open: true,
            size: metadata.len(),
        })
    }
}

pub const IO_ERR: i32 = LIBCERROR_ERROR_DOMAINS_LIBCERROR_ERROR_DOMAIN_IO as i32;
pub const ARGUMENT_ERR: i32 = LIBCERROR_ERROR_DOMAINS_LIBCERROR_ERROR_DOMAIN_ARGUMENTS as i32;

#[no_mangle]
pub unsafe extern "C" fn io_handle_free(
    io_handle: *mut *mut IoHandle,
    _error: *mut LibbfioErrorRefMut,
) -> c_int {
    trace!("io_handle_free");
    Box::from_raw(*io_handle);

    1 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_read(
    io_handle: *mut IoHandle,
    buffer: *mut u8,
    size: usize,
    error: *mut LibbfioErrorRefMut,
) -> isize {
    trace!("io_handle_read");

    let s = slice::from_raw_parts_mut(buffer, size);
    match (*io_handle).inner.read(s) {
        Ok(cnt) => cnt as isize,
        Err(e) => {
            libcerror_error_set(
                error as _,
                IO_ERR,
                LIBCERROR_IO_ERROR_LIBCERROR_IO_ERROR_READ_FAILED as i32,
                CString::new("%s.").unwrap().into_raw(),
                CString::new(format!("io_handle_read: {:?}", e))
                    .unwrap()
                    .into_raw(),
            );
            return -1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_write(
    io_handle: *mut IoHandle,
    buffer: *const u8,
    size: usize,
    error: *mut LibbfioErrorRefMut,
) -> isize {
    trace!("io_handle_write");

    let s = slice::from_raw_parts(buffer, size);
    match (*io_handle).inner.write(s) {
        Ok(cnt) => cnt as isize,
        Err(e) => {
            libcerror_error_set(
                error as _,
                IO_ERR,
                LIBCERROR_IO_ERROR_LIBCERROR_IO_ERROR_WRITE_FAILED as i32,
                CString::new("%s.").unwrap().into_raw(),
                CString::new(format!("io_handle_write: {:?}", e))
                    .unwrap()
                    .into_raw(),
            );
            return -1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_is_open(
    io_handle: *mut IoHandle,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    trace!("io_handle_is_open");
    match (*io_handle).is_open {
        true => 1,
        false => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_seek(
    io_handle: *mut IoHandle,
    offset: u64,
    whence: c_int,
    error: *mut LibbfioErrorRefMut,
) -> u64 {
    trace!("io_handle_seek");

    let seek_from = match whence as u32 {
        SEEK_SET => SeekFrom::Start(offset),
        SEEK_END => SeekFrom::End(offset as i64),
        SEEK_CUR => SeekFrom::Current(offset as i64),
        _ => {
            libcerror_error_set(
                error as _,
                ARGUMENT_ERR,
                LIBCERROR_ARGUMENT_ERROR_LIBCERROR_ARGUMENT_ERROR_INVALID_VALUE as i32,
                CString::new("%s: invalid whence.").unwrap().into_raw(),
                CString::new("io_handle_seek").unwrap().into_raw(),
            );
            return 0;
        }
    };

    match (*io_handle).inner.seek(seek_from) {
        Ok(count) => count,
        Err(e) => {
            libcerror_error_set(
                error as _,
                IO_ERR,
                LIBCERROR_IO_ERROR_LIBCERROR_IO_ERROR_SEEK_FAILED as i32,
                CString::new("%s.").unwrap().into_raw(),
                CString::new(format!("io_handle_seek: {:?}", e))
                    .unwrap()
                    .into_raw(),
            );
            return 0;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_get_size(
    io_handle: *mut IoHandle,
    size: *mut size64_t,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    trace!("io_handle_get_size");

    (*io_handle).size as c_int
}
