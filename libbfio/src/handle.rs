use crate::error::Error;
use crate::ffi_error::LibbfioErrorRefMut;
use crate::io_handle::IoHandle;
use crate::io_handle::*;
use libyal_rs_common::ffi::AsTypeRef;

use libbfio_sys::{size64_t, SEEK_CUR, SEEK_END, SEEK_SET};
use std::convert::TryFrom;

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
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

pub type IoHandleConstRef = *const IoHandle;
pub type IoHandleRefMut = *mut IoHandle;
pub type BoxedIoHandleRefMut = *mut *mut IoHandle;

//enum LibbfioFlags {
//    /* The IO handle is not managed by the library
//     */
//    IoHandleNonManaged = 0x00,
//
//    /* The IO handle is managed by the library
//     */
//    IoHandleManaged = 0x01,
//
//    /* The IO handle is cloned by the function
//     */
//    CloneByFunction = 0x00,
//
//    /* The IO handle is not cloned, but passed as a reference
//     */
//    CloneByReference = 0x02,
//}

/* The access flags definitions
 * bit 1						set to 1 for read access
 * bit 2						set to 1 for write access
 * bit 3						set to 1 to truncate an existing file on write
 * bit 4-8						not used
 */
pub enum LibbfioAccessFlags {
    Read = 0x01,
    Write = 0x02,
    Truncate = 0x04,
}

impl LibbfioAccessFlags {
    pub fn to_int(&self) -> c_int {
        match self {
            LibbfioAccessFlags::Read => 0x01,
            LibbfioAccessFlags::Write => 0x02,
            LibbfioAccessFlags::Truncate => 0x04,
        }
    }
}

extern "C" {
    pub fn libbfio_handle_initialize(
        handle: *mut HandleRefMut,
        io_handle: IoHandleRefMut,
        free_io_handle: Option<
            unsafe extern "C" fn(
                io_handle: *mut IoHandleRefMut,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        clone_io_handle: Option<
            unsafe extern "C" fn(
                destination_io_handle: BoxedIoHandleRefMut,
                source_io_handle: IoHandleRefMut,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        open: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                access_flags: c_int,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        close: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        read: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                buffer: *mut u8,
                size: usize,
                error: *mut LibbfioErrorRefMut,
            ) -> isize,
        >,
        write: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                buffer: *const u8,
                size: usize,
                error: *mut LibbfioErrorRefMut,
            ) -> isize,
        >,
        seek_offset: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                offset: u64,
                whence: c_int,
                error: *mut LibbfioErrorRefMut,
            ) -> u64,
        >,
        exists: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        is_open: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
                error: *mut LibbfioErrorRefMut,
            ) -> c_int,
        >,
        get_size: Option<
            unsafe extern "C" fn(
                io_handle: IoHandleRefMut,
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

impl Handle {
    pub fn open_file(path: impl AsRef<Path>, flags: LibbfioAccessFlags) -> Result<Handle, Error> {
        let f = match flags {
            LibbfioAccessFlags::Read => OpenOptions::new().read(true).open(path),
            LibbfioAccessFlags::Write => OpenOptions::new().write(true).open(path),
            LibbfioAccessFlags::Truncate => OpenOptions::new().create(true).open(path),
        };

        let mut handle = ptr::null_mut();
        let mut error = ptr::null_mut();

        let io_handle = Box::new(f.expect("failed to open file")) as Box<dyn RwSeek>;
        let heap_ptr = Box::into_raw(Box::new(io_handle));

        let retcode = unsafe {
            libbfio_handle_initialize(
                &mut handle as _,
                heap_ptr,
                Some(io_handle_free),
                None,
                None,
                None,
                Some(io_handle_read),
                Some(io_handle_write),
                Some(io_handle_seek),
                None,
                None,
                Some(io_handle_get_size),
                // LibbfioFlagIoHandleManaged
                1,
                &mut error,
            )
        };

        if retcode != 1 {
            Err(Error::try_from(error)?)
        } else {
            let mut err = ptr::null_mut();
            if unsafe { libbfio_handle_set_access_flags(handle, flags.to_int(), &mut err) } != 1 {
                return Err(Error::try_from(err)?);
            }
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

        if !(error.is_null()) {
            let ffi_err = Error::try_from(error);

            let io_err = match ffi_err {
                Ok(e) => io::Error::new(io::ErrorKind::Other, format!("{}", e)),
                Err(_e) => io::Error::new(
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

impl Write for Handle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut flags = 0_i32;
        let mut error = ptr::null_mut();
        if unsafe { libbfio_handle_get_access_flags(self.as_type_ref(), &mut flags, &mut error) }
            != 1
        {
            let ffi_err = Error::try_from(error);

            let io_err = match ffi_err {
                Ok(e) => io::Error::new(io::ErrorKind::Other, format!("{}", e)),
                Err(_e) => io::Error::new(
                    io::ErrorKind::Other,
                    format!("error while getting error information"),
                ),
            };

            return Err(io_err);
        }

        if flags & LibbfioAccessFlags::Write.to_int() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("file is not open for writing"),
            ));
        };

        let mut error = ptr::null_mut();
        let write_count = unsafe {
            libbfio_handle_write_buffer(self.as_type_ref(), buf.as_ptr(), buf.len(), &mut error)
        };

        if !(error.is_null()) {
            let ffi_err = Error::try_from(error);

            let io_err = match ffi_err {
                Ok(e) => io::Error::new(io::ErrorKind::Other, format!("{}", e)),
                Err(_e) => io::Error::new(
                    io::ErrorKind::Other,
                    format!("error while getting error information"),
                ),
            };

            Err(io_err)
        } else {
            Ok(write_count as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for Handle {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let mut error = ptr::null_mut();
        let seek_count = unsafe {
            match pos {
                SeekFrom::Current(p) => libbfio_handle_seek_offset(
                    self.as_type_ref(),
                    p as u64,
                    SEEK_CUR as c_int,
                    &mut error,
                ),
                SeekFrom::End(p) => libbfio_handle_seek_offset(
                    self.as_type_ref(),
                    p as u64,
                    SEEK_END as c_int,
                    &mut error,
                ),
                SeekFrom::Start(p) => libbfio_handle_seek_offset(
                    self.as_type_ref(),
                    p as u64,
                    SEEK_SET as c_int,
                    &mut error,
                ),
            }
        };

        if !(error.is_null()) {
            let ffi_err = Error::try_from(error);

            let io_err = match ffi_err {
                Ok(e) => io::Error::new(io::ErrorKind::Other, format!("{}", e)),
                Err(_e) => io::Error::new(
                    io::ErrorKind::Other,
                    format!("error while getting error information"),
                ),
            };

            Err(io_err)
        } else {
            Ok(seek_count as u64)
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        use libyal_rs_common::ffi::AsTypeRef;
        use log::trace;

        let mut error = ptr::null_mut();

        trace!("Calling `libbfio_handle_free`");

        unsafe {
            libbfio_handle_free(&mut self.as_type_ref_mut() as *mut _, &mut error);
        }

        trace!("Called `libbfio_handle_free`");

        if !(error.is_null()) {
            let e = Error::try_from(error).expect("Failed to read error");
            dbg!(e);
        }

        debug_assert!(error.is_null(), "`{}` failed!", module_path!());
    }
}

#[cfg(test)]
mod tests {
    use crate::handle::{Handle, LibbfioAccessFlags};

    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};

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
    fn test_read() {
        let tmp_dir = tmp_src_dir();
        let test_file = test_file(&tmp_dir, Some(FILE_CONTENT));
        let test_file_path = tmp_dir.path().join(test_file).canonicalize().unwrap();

        let mut handle = Handle::open_file(test_file_path, LibbfioAccessFlags::Read).unwrap();
        let mut buf = vec![];

        handle.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, FILE_CONTENT);
    }

    #[test]
    fn test_write() {
        let tmp_dir = tmp_src_dir();
        let test_file = test_file(&tmp_dir, Some(FILE_CONTENT));
        let test_file_path = tmp_dir.path().join(test_file).canonicalize().unwrap();

        let mut handle = Handle::open_file(&test_file_path, LibbfioAccessFlags::Write).unwrap();

        handle.write(b"Hello").unwrap();

        let mut new = File::open(test_file_path).unwrap();
        let mut buf = vec![];
        new.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, FILE_CONTENT);
    }

    #[test]
    fn test_write_checks_access_flags() {
        let tmp_dir = tmp_src_dir();
        let test_file = test_file(&tmp_dir, Some(FILE_CONTENT));
        let test_file_path = tmp_dir.path().join(test_file).canonicalize().unwrap();

        let mut handle = Handle::open_file(&test_file_path, LibbfioAccessFlags::Read).unwrap();

        assert!(handle.write(b"Hello").is_err());
    }


    #[test]
    fn test_seek() {
        let tmp_dir = tmp_src_dir();
        let test_file = test_file(&tmp_dir, Some(FILE_CONTENT));
        let test_file_path = tmp_dir.path().join(test_file).canonicalize().unwrap();

        let mut handle = Handle::open_file(test_file_path, LibbfioAccessFlags::Read).unwrap();
        let mut buf = vec![];

        handle.seek(SeekFrom::Current(2)).unwrap();
        handle.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, &FILE_CONTENT[2..]);
    }
}
