use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use crate::libbfio::{size64_t, SEEK_CUR, SEEK_END, SEEK_SET};
use libyal_rs_common::ffi::AsTypeRef;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::c_int;
use std::path::Path;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct IoHandle<I: Read + Write + Seek + Clone> {
    inner: Box<I>,
    open: bool,
}

impl IoHandle<Arc<RwLock<File>>> {
    pub fn open_file(p: impl AsRef<Path>) -> Result<Self, Error> {
        IoHandle {
            inner: Arc::new(File::open(p)?),
            open: false,
        }
    }
}

pub unsafe extern "C" fn io_handle_free<I: Read + Seek + Seek + Clone>(
    io_handle: *mut IoHandle<I>,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    Box::from_raw(io_handle);
    1 as c_int
}

pub unsafe extern "C" fn io_handle_open<I: Read + Seek + Seek + Clone>(
    mut io_handle: IoHandle<I>,
    access_flags: c_int,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    io_handle.open = true;
    1 as c_int
}

pub unsafe extern "C" fn io_handle_clone<I: Read + Seek + Seek + Clone>(
    mut destination_io_handle: *mut IoHandle<I>,
    source_io_handle: IoHandle<I>,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    *source_io_handle.clone_into(&mut destination_io_handle);
    1 as c_int
}

pub unsafe extern "C" fn io_handle_read<I: Read + Seek + Seek + Clone>(
    mut io_handle: IoHandle<I>,
    mut buffer: *mut u8,
    size: usize,
    error: *mut LibbfioErrorRefMut,
) -> isize {
    if !io_handle.open {
        // TODO: error
        return 0;
    }
    let mut temp = vec![0; size];

    io_handle.inner.read(&mut temp).expect("read failed") as isize
}

pub unsafe extern "C" fn io_handle_write<I: Read + Seek + Seek + Clone>(
    mut io_handle: IoHandle<I>,
    buffer: *const u8,
    size: usize,
    error: *mut LibbfioErrorRefMut,
) -> isize {
    if !io_handle.open {
        // TODO: error
        return 0;
    }
    let mut slice = vec![0_u8; size];
    buffer.copy_to(slice.as_mut_ptr(), size);

    io_handle.inner.write(&slice).expect("write failed") as isize
}

pub unsafe extern "C" fn io_handle_seek<I: Read + Seek + Seek + Clone>(
    mut io_handle: IoHandle<I>,
    offset: u64,
    whence: c_int,
    error: *mut LibbfioErrorRefMut,
) -> u64 {
    if !io_handle.open {
        // TODO: error
        return 0;
    }

    let seek_from = match whence {
        SEEK_SET => SeekFrom::Start(offset),
        SEEK_END => SeekFrom::End(offset as i64),
        SEEK_CUR => SeekFrom::Current(offset as i64),
        _ => panic!("unexpected `whence`"),
    };

    io_handle.inner.seek(seek_from).expect("Seek failed")
}

pub unsafe extern "C" fn io_handle_close<I: Read + Seek + Seek + Clone>(
    mut io_handle: IoHandle<I>,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    io_handle.open = false;
    1 as c_int
}

pub unsafe extern "C" fn io_handle_is_open<I: Read + Seek + Seek + Clone>(
    io_handle: IoHandle<I>,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    io_handle.open;
    1
}

pub unsafe extern "C" fn io_handle_exists<I: Read + Seek + Seek + Clone>(
    io_handle: IoHandle<I>,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    1
}

pub unsafe extern "C" fn io_handle_get_size<I: Read + Seek + Seek + Clone>(
    mut io_handle: IoHandle<I>,
    size: *mut size64_t,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    size.write(io_handle.inner.stream_len().expect("get size failed"));
    1
}
