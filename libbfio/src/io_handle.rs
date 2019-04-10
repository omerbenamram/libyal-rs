use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use libbfio_sys::{size64_t, SEEK_CUR, SEEK_END, SEEK_SET};
use log::trace;

use crate::handle::{BoxedIoHandleRefMut, IoHandleRefMut};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::c_int;
use std::path::Path;
use std::slice;

pub trait RwSeek: Read + Write + Seek {}
impl<T: Read + Write + Seek> RwSeek for T {}

pub type IoHandle = Box<dyn RwSeek>;

#[no_mangle]
pub unsafe extern "C" fn io_handle_free(
    io_handle: BoxedIoHandleRefMut,
    _error: *mut LibbfioErrorRefMut,
) -> c_int {
    trace!("io_handle_free");
    Box::from_raw(*io_handle);

    1 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_read(
    io_handle: IoHandleRefMut,
    buffer: *mut u8,
    size: usize,
    _error: *mut LibbfioErrorRefMut,
) -> isize {
    trace!("io_handle_read");

    let s = slice::from_raw_parts_mut(buffer, size);
    (*io_handle).read(s).expect("read failed") as isize
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_write(
    io_handle: IoHandleRefMut,
    buffer: *const u8,
    size: usize,
    _error: *mut LibbfioErrorRefMut,
) -> isize {
    trace!("io_handle_write");

    let s = slice::from_raw_parts(buffer, size);
    (*io_handle).write(s).expect("write failed") as isize
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_seek(
    io_handle: IoHandleRefMut,
    offset: u64,
    whence: c_int,
    _error: *mut LibbfioErrorRefMut,
) -> u64 {
    trace!("io_handle_seek");

    let seek_from = match whence as u32 {
        SEEK_SET => SeekFrom::Start(offset),
        SEEK_END => SeekFrom::End(offset as i64),
        SEEK_CUR => SeekFrom::Current(offset as i64),
        _ => panic!("unexpected `whence`"),
    };

    (*io_handle).seek(seek_from).expect("Seek failed")
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_get_size(
    io_handle: IoHandleRefMut,
    size: *mut size64_t,
    _error: *mut LibbfioErrorRefMut,
) -> c_int {
    trace!("io_handle_get_size");
    *size = (*io_handle).stream_len().expect("get size failed");

    1
}
