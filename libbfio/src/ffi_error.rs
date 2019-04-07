use crate::error::Error;
use crate::libbfio::FILE;
use libyal_rs_common::ffi::AsTypeRef;
use log::trace;
use std::convert::TryFrom;
use std::ffi::{c_void, CStr};
use std::fmt::{self, Display, Formatter};
use std::mem;

pub struct __LibbfioError(isize);

pub type LibbfioErrorRefMut = *mut __LibbfioError;
pub type LibbfioErrorRef = *const __LibbfioError;

pub struct LibbfioError(LibbfioErrorRefMut);

impl AsTypeRef for LibbfioError {
    type Ref = LibbfioErrorRef;
    type RefMut = LibbfioErrorRefMut;

    fn as_type_ref(&self) -> Self::Ref {
        self.0 as *const _
    }

    fn as_type_ref_mut(&mut self) -> Self::RefMut {
        self.0
    }

    fn as_raw(&mut self) -> *mut Self::RefMut {
        &mut self.0 as *mut _
    }
}

extern "C" {
    pub fn libbfio_error_free(error: *mut LibbfioErrorRefMut);

    pub fn libbfio_error_fprint(error: LibbfioErrorRef, stream: *mut FILE)
        -> ::std::os::raw::c_int;

    pub fn libbfio_error_sprint(
        error: LibbfioErrorRef,
        string: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;

    pub fn libbfio_error_backtrace_fprint(
        error: LibbfioErrorRef,
        stream: *mut FILE,
    ) -> ::std::os::raw::c_int;

    pub fn libbfio_error_backtrace_sprint(
        error: LibbfioErrorRef,
        string: *mut ::std::os::raw::c_char,
        size: usize,
    ) -> ::std::os::raw::c_int;
}

impl Drop for LibbfioError {
    fn drop(&mut self) {
        trace!("Calling `libbfio_error_free`");

        unsafe { libbfio_error_free(self.as_raw()) };
    }
}

impl TryFrom<*mut __LibbfioError> for Error {
    type Error = Error;

    fn try_from(err: *mut __LibbfioError) -> Result<Self, Self::Error> {
        if err.is_null() {
            return Err(Error::Other("Error pointer cannot be NULL".to_owned()));
        }

        let mut buffer = vec![0; 1024];

        let retcode =
            unsafe { libbfio_error_sprint(err as *const _, buffer.as_mut_ptr(), buffer.len()) };

        if retcode == -1 {
            Err(Error::FFI("Failed to print error".to_owned()))
        } else {
            let repr = unsafe { CStr::from_ptr(buffer.as_ptr()) };
            Ok(Error::FFI(repr.to_string_lossy().to_string()))
        }
    }
}
