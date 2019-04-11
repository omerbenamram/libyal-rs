use crate::error::Error;
use libyal_rs_common::ffi::AsTypeRef;
use libfsntfs_sys::FILE;
use log::trace;
use std::convert::TryFrom;
use std::ffi::{c_void, CStr};
use std::fmt::{self, Display, Formatter};
use std::mem;

#[repr(C)]
pub struct __LibfsntfsError(isize);

pub type LibfsntfsErrorRefMut = *mut __LibfsntfsError;
pub type LibfsntfsErrorRef = *const __LibfsntfsError;

#[repr(C)]
pub struct LibfsntfsError(LibfsntfsErrorRefMut);

impl AsTypeRef for LibfsntfsError {
    type Ref = LibfsntfsErrorRef;
    type RefMut = LibfsntfsErrorRefMut;

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

extern "C" {
    pub fn libfsntfs_error_free(error: *mut LibfsntfsErrorRefMut);
    pub fn libfsntfs_error_fprint(
        error: LibfsntfsErrorRef,
        stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn libfsntfs_error_sprint(
        error: LibfsntfsErrorRef,
        string: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;
    pub fn libfsntfs_error_backtrace_fprint(
        error: LibfsntfsErrorRef,
        stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn libfsntfs_error_backtrace_sprint(
        error: LibfsntfsErrorRef,
        string: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;
}

impl Drop for LibfsntfsError {
    fn drop(&mut self) {
        trace!("Calling `libfsntfs_error_free`");

        unsafe { libfsntfs_error_free(self.as_raw()) };
    }
}

impl TryFrom<*mut __LibfsntfsError> for Error {
    type Error = Error;

    fn try_from(err: *mut __LibfsntfsError) -> Result<Self, Self::Error> {
        if err.is_null() {
            return Err(Error::Other("Error pointer cannot be NULL".to_owned()));
        }

        let mut buffer = vec![0; 1024];

        let retcode =
            unsafe { libfsntfs_error_backtrace_sprint(err as *const _, buffer.as_mut_ptr(), buffer.len()) };

        if retcode == -1 {
            Err(Error::FFI("Failed to print error".to_owned()))
        } else {
            let repr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
            Ok(Error::FFI(repr.to_string_lossy().to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::fixtures::*;
    use crate::volume::{AccessMode, Volume};

    #[test]
    fn test_error() {
        let result = Volume::open("non-existent", AccessMode::Read);
        assert!(result.is_err());

        if let Err(e) = result {
            if let Error::FFI(s) = e {
                dbg!(&s);
                assert!(
                    s.find("libfsntfs_volume_open").is_some(),
                    "should contain FFI function name"
                );
                assert!(
                    s.find("unable to open volume").is_some(),
                    "should contain message string"
                );

                return;
            }
        }

        panic!("Test should not reach here!");
    }
}
