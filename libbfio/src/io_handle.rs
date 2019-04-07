use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use crate::libbfio::{SEEK_CUR, SEEK_END, SEEK_SET, size64_t};
use libyal_rs_common::ffi::AsTypeRef;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::c_int;
use std::path::Path;

#[derive(Clone)]
pub struct IoHandle<I: Read + Write + Seek + Clone> {
    inner: Box<I>,
    open: bool,
}

impl IoHandle<File> {
    pub fn open_file(p: impl AsRef<Path>) -> Result<Self, Error> {
        IoHandle {
            inner: File::open(p)?,
            open: false,
        }
    }
}

impl<I: Read + Write + Seek + Clone> IoHandle<I> {
    unsafe extern "C" fn io_handle_free(
        io_handle: *mut IoHandle<I>,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int {
        Box::from_raw(io_handle);
        1 as c_int
    }

    unsafe extern "C" fn io_handle_open(
        mut io_handle: IoHandle<I>,
        access_flags: c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int {
        io_handle.open = true;
        1 as c_int
    }

    unsafe extern "C" fn io_handle_clone(
        mut destination_io_handle: *mut IoHandle<I>,
        source_io_handle: IoHandle<I>,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int {
        *source_io_handle.clone_into(&mut destination_io_handle);
        1 as c_int
    }

    unsafe extern "C" fn io_handle_read(
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

    unsafe extern "C" fn io_handle_write(
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

    unsafe extern "C" fn io_handle_seek(
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

    unsafe extern "C" fn io_handle_close(
        mut io_handle: IoHandle<I>,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int {
        io_handle.open = false;
        1 as c_int
    }

    unsafe extern "C" fn is_open(io_handle: IoHandle<I>, error: *mut LibbfioErrorRefMut) -> c_int {
        io_handle.open;
        1
    }

    unsafe extern "C" fn exists(io_handle: IoHandle<I>, error: *mut LibbfioErrorRefMut) -> c_int {
        1
    }

    unsafe extern "C" fn get_size(
        mut io_handle: IoHandle<I>,
        size: *mut size64_t,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int {
        size.write(io_handle.inner.stream_len().expect("get size failed"));
        1
    }
}
