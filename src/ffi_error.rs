use crate::error::Error;
use crate::ffi::AsTypeRef;
use crate::libfsntfs::FILE;
use std::convert::TryFrom;
use std::ffi::{c_void, CStr};
use std::fmt::{self, Display, Formatter};
use std::mem;

#[repr(C)]
pub struct __LibfsntfsError(c_void);

pub type LibfsntfsErrorRef = *mut __LibfsntfsError;

declare_ffi_type!(LibfsntfsError, LibfsntfsErrorRef);
impl_ffi_type!(LibfsntfsError, LibfsntfsErrorRef);

extern "C" {
    #[link_name = "\u{1}_libfsntfs_error_free"]
    pub fn libfsntfs_error_free(error: *mut LibfsntfsErrorRef);
    #[link_name = "\u{1}_libfsntfs_error_fprint"]
    pub fn libfsntfs_error_fprint(
        error: LibfsntfsErrorRef,
        stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_error_sprint"]
    pub fn libfsntfs_error_sprint(
        error: LibfsntfsErrorRef,
        string: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_error_backtrace_fprint"]
    pub fn libfsntfs_error_backtrace_fprint(
        error: LibfsntfsErrorRef,
        stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}_libfsntfs_error_backtrace_sprint"]
    pub fn libfsntfs_error_backtrace_sprint(
        error: LibfsntfsErrorRef,
        string: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;
}

impl Drop for LibfsntfsError {
    fn drop(&mut self) {
        unsafe { libfsntfs_error_free(&mut self.as_type_ref() as *mut _) };
    }
}

impl TryFrom<LibfsntfsErrorRef> for Error {
    type Error = Error;

    fn try_from(err: LibfsntfsErrorRef) -> Result<Self, Self::Error> {
        if err.is_null() {
            return Err(Error::Other("No Error".to_owned()));
        }

        let mut buffer = vec![0; 1024];

        let retcode = unsafe { libfsntfs_error_sprint(err, buffer.as_mut_ptr(), buffer.len()) };

        if retcode == -1 {
            Err(Error::FFI("Failed to print error".to_owned()))
        } else {
            let repr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
            Ok(Error::FFI(repr.to_string_lossy().to_string()))
        }
    }
}
