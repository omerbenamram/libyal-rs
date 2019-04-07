use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use libbfio_sys::{size64_t, SEEK_CUR, SEEK_END, SEEK_SET};

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::c_int;
use std::path::Path;
use std::sync::{Arc, RwLock};

pub trait RwSeek: Read + Write + Seek {}
impl<T: Read + Write + Seek> RwSeek for T {}

#[repr(C)]
pub struct IoHandle {
    inner: Box<dyn RwSeek>,
    open: bool,
}

//impl Clone for IoHandle {
//    fn clone(&self) -> Self {
//        IoHandle {
//            inner: Box::new(self.inner),
//            open: self.open,
//        }
//    }
//}

impl IoHandle {
    pub fn open_file(p: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(IoHandle {
            inner: Box::new(File::open(p).expect("Failed to open file")) as Box<dyn RwSeek>,
            open: false,
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_free(
    io_handle: *mut *mut IoHandle,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    Box::from_raw(*io_handle);
    1 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_open(
    mut io_handle: *mut IoHandle,
    access_flags: c_int,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    (*io_handle).open = true;
    1 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_clone(
    destination_io_handle: *mut *mut IoHandle,
    source_io_handle: *mut IoHandle,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
//    source_io_handle.clone_into(&mut (*destination_io_handle));
    1 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_read(
    mut io_handle: *mut IoHandle,
    mut buffer: *mut u8,
    size: usize,
    error: *mut LibbfioErrorRefMut,
) -> isize {
    if !(*io_handle).open {
        // TODO: error
        return 0;
    }
    let mut temp = vec![0; size];

    (*io_handle).inner.read(&mut temp).expect("read failed") as isize
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_write(
    mut io_handle: *mut IoHandle,
    buffer: *const u8,
    size: usize,
    error: *mut LibbfioErrorRefMut,
) -> isize {
    if !(*io_handle).open {
        // TODO: error
        return 0;
    }
    let mut slice = vec![0_u8; size];
    buffer.copy_to(slice.as_mut_ptr(), size);

    (*io_handle).inner.write(&slice).expect("write failed") as isize
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_seek(
    mut io_handle: *mut IoHandle,
    offset: u64,
    whence: c_int,
    error: *mut LibbfioErrorRefMut,
) -> u64 {
    if !(*io_handle).open {
        // TODO: error
        return 0;
    }

    let seek_from = match whence as u32 {
        SEEK_SET => SeekFrom::Start(offset),
        SEEK_END => SeekFrom::End(offset as i64),
        SEEK_CUR => SeekFrom::Current(offset as i64),
        _ => panic!("unexpected `whence`"),
    };

    (*io_handle).inner.seek(seek_from).expect("Seek failed")
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_close(
    mut io_handle: *mut IoHandle,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    (*io_handle).open = false;
    1 as c_int
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_is_open(
    io_handle: *mut IoHandle,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    (*io_handle).open;
    1
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_exists(
    io_handle: *mut IoHandle,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn io_handle_get_size(
    mut io_handle: *mut IoHandle,
    size: *mut size64_t,
    error: *mut LibbfioErrorRefMut,
) -> c_int {
    size.write((*io_handle).inner.stream_len().expect("get size failed"));
    1
}
