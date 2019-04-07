use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use crate::io_handle::IoHandle;
use crate::io_handle::*;
use libyal_rs_common::ffi::AsTypeRef;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::os::raw::c_int;
use std::path::Path;
use std::{io, ptr};

#[repr(C)]
pub struct __Handle(isize);

pub type HandleRefMut = *mut __Handle;
pub type HandleRef = *const __Handle;

#[repr(C)]
pub struct Handle(HandleRefMut);

impl AsTypeRef for Handle {
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

impl Handle {
    pub fn wrap_ptr(ptr: HandleRefMut) -> Self {
        Handle(ptr)
    }
}

type io_handle_t = *mut IoHandle;

extern "C" {
    pub fn libbfio_handle_initialize(
        handle: *mut HandleRefMut,
        io_handle: io_handle_t,
        free_io_handle: Option<
            unsafe extern "C" fn(
                io_handle: *mut io_handle_t,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        clone_io_handle: Option<
            unsafe extern "C" fn(
                destination_io_handle: *mut io_handle_t,
                source_io_handle: io_handle_t,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        open: Option<
            unsafe extern "C" fn(
                io_handle: io_handle_t,
                access_flags: c_int,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        close: Option<
            unsafe extern "C" fn(io_handle: io_handle_t, error: *mut LibbfioErrorRefMut) -> c_int,
        >,
        read: Option<
            unsafe extern "C" fn(
                io_handle: io_handle_t,
                buffer: *mut u8,
                size: usize,
                error: *mut LibbfioErrorRefMut,
            ) -> isize,
        >,
        write: Option<
            unsafe extern "C" fn(
                io_handle: io_handle_t,
                buffer: *const u8,
                size: usize,
                error: *mut LibbfioErrorRefMut,
            ) -> isize,
        >,
        seek_offset: Option<
            unsafe extern "C" fn(
                io_handle: io_handle_t,
                offset: u64,
                whence: c_int,
                error: *mut LibbfioErrorRefMut,
            ) -> u64,
        >,
        exists: Option<
            unsafe extern "C" fn(io_handle: io_handle_t, error: *mut LibbfioErrorRefMut) -> c_int,
        >,
        is_open: Option<
            unsafe extern "C" fn(io_handle: io_handle_t, error: *mut LibbfioErrorRefMut) -> c_int,
        >,
        get_size: Option<
            unsafe extern "C" fn(
                io_handle: io_handle_t,
                size: *mut u64,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        flags: u8,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;

    pub fn libbfio_handle_free(handle: *mut HandleRefMut, error: *mut LibbfioErrorRefMut) -> c_int;
    pub fn libbfio_handle_clone(
        destination_handle: *mut HandleRefMut,
        source_handle: HandleRef,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_open(
        handle: HandleRef,
        access_flags: c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_reopen(
        handle: HandleRef,
        access_flags: c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_close(handle: HandleRef, error: *mut LibbfioErrorRefMut) -> c_int;
    pub fn libbfio_handle_read_buffer(
        handle: HandleRef,
        buffer: *mut u8,
        size: usize,
        error: *mut LibbfioErrorRefMut,
    ) -> isize;
    pub fn libbfio_handle_write_buffer(
        handle: HandleRef,
        buffer: *const u8,
        size: usize,
        error: *mut LibbfioErrorRefMut,
    ) -> isize;
    pub fn libbfio_handle_seek_offset(
        handle: HandleRef,
        offset: u64,
        whence: c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> u64;
    pub fn libbfio_handle_exists(handle: HandleRef, error: *mut LibbfioErrorRefMut) -> c_int;
    pub fn libbfio_handle_is_open(handle: HandleRef, error: *mut LibbfioErrorRefMut) -> c_int;
    pub fn libbfio_handle_get_io_handle(
        handle: HandleRef,
        io_handle: *mut HandleRefMut,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_get_access_flags(
        handle: HandleRef,
        access_flags: *mut c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_set_access_flags(
        handle: HandleRef,
        access_flags: c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_get_offset(
        handle: HandleRef,
        offset: *mut u64,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_get_size(
        handle: HandleRef,
        size: *mut u64,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_set_open_on_demand(
        handle: HandleRef,
        open_on_demand: u8,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_set_track_offsets_read(
        handle: HandleRef,
        track_offsets_read: u8,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_get_number_of_offsets_read(
        handle: HandleRef,
        number_of_read_offsets: *mut c_int,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
    pub fn libbfio_handle_get_offset_read(
        handle: HandleRef,
        index: c_int,
        offset: *mut u64,
        size: *mut u64,
        error: *mut LibbfioErrorRefMut,
    ) -> c_int;
}

impl Drop for Handle {
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

impl Handle {
    pub fn open_file(path: impl AsRef<Path>) -> Result<Handle, Error> {
        let mut handle = ptr::null_mut();
        let mut error = ptr::null_mut();

        let io_handle = IoHandle::open_file(path).expect("open file");
        let boxed_handle = Box::new(io_handle);

        let retcode = unsafe {
            libbfio_handle_initialize(
                &mut handle as _,
                Box::into_raw(boxed_handle),
                Some(io_handle_free),
                Some(io_handle_clone),
                Some(io_handle_open),
                Some(io_handle_close),
                Some(io_handle_read),
                Some(io_handle_write),
                Some(io_handle_seek),
                Some(io_handle_exists),
                Some(io_handle_is_open),
                Some(io_handle_get_size),
                0,
                &mut error,
            )
        };

        if retcode != 1 {
            Err(Error::try_from(error)?)
        } else {
            Ok(Handle::wrap_ptr(handle))
        }
    }
}

impl Read for Handle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut error = ptr::null_mut();
        let read_count = unsafe {
            libbfio_handle_read_buffer(self.as_type_ref(), buf.as_mut_ptr(), buf.len(), &mut error)
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

#[cfg(test)]
mod tests {
    use crate::handle::Handle;

    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use tempdir::TempDir;

    const TMP_FILE_NAME: &str = "a.txt";
    const FILE_CONTENT: &[u8; 9] = b"some_data";

    fn tmp_src_dir() -> TempDir {
        let tmp_dir = TempDir::new("test").unwrap();
        tmp_dir
    }

    fn test_file(tmp_dir: &TempDir, content: Option<&[u8]>) -> &'static str {
        let mut tmp_file = File::create(tmp_dir.path().join(TMP_FILE_NAME)).unwrap();

        match content {
            Some(content) => {
                tmp_file.write(content).unwrap();
            }
            None => {}
        };

        TMP_FILE_NAME
    }

    #[test]
    fn it_works() {
        let tmp_dir = tmp_src_dir();
        let test_file = test_file(&tmp_dir, Some(FILE_CONTENT));
        let test_file_path = tmp_dir.path().join(test_file).canonicalize().unwrap();

        let mut handle = Handle::open_file(test_file_path).unwrap();
        let mut buf = vec![0; 10];

        handle.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, FILE_CONTENT);
    }
}
