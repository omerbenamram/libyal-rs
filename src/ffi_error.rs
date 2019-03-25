use crate::ffi::AsFFIPtr;
use crate::libfsntfs::*;
use std::ffi::c_void;
use std::fmt::{self, Display, Formatter};

#[repr(C)]
pub struct __LibfsntfsError(c_void);

pub type LibfsntfsErrorRef = *mut __LibfsntfsError;

declare_ffi_type!(LibfsntfsError, LibfsntfsErrorRef);
impl_ffi_type!(LibfsntfsError, LibfsntfsErrorRef);

impl Drop for LibfsntfsError {
    fn drop(&mut self) {
        unsafe { libfsntfs_error_free(self.as_ffi_ptr()) };
    }
}

impl LibfsntfsErrorRef {
    fn as_string(&self) -> Result<String, String> {
        let mut buffer = vec![0; 1024];

        let retcode =
            unsafe { libfsntfs_error_sprint(self as *mut _, buffer.as_mut_ptr(), buffer.len()) };

        if retcode == -1 {
            Err("Failed to print error".to_owned())
        } else {
            Ok(
                String::from_utf8(buffer.into_iter().map(|c| c as u8).collect())
                    .expect("invalid UTF-8"),
            )
        }
    }
}
