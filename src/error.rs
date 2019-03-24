use crate::ffi::AsFFIPtr;
use crate::libfsntfs::*;
use std::fmt::{self, Display, Formatter, Debug};
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;

pub struct LibfsntfsError {
    code: isize,
}

impl LibfsntfsError {
    pub fn new() -> Self {
        LibfsntfsError { code: 0 }
    }

    pub fn is_error(&self) -> bool {
        self.code == 0
    }
}

impl_as_ffi_ptr!(isize, code, LibfsntfsError);

impl Drop for LibfsntfsError {
    fn drop(&mut self) {
        unsafe { libfsntfs_error_free(self.as_ffi_ptr()) };
    }
}

impl Debug for LibfsntfsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut buffer = vec![0; 1024];
        let mut _code = self.code.clone();

        let fmt = unsafe {
            libfsntfs_error_sprint(&mut _code as *mut _, buffer.as_mut_ptr(), buffer.len())
        };

        f.write_str(
            &String::from_utf8(buffer.into_iter().map(|c| c as u8).collect())
                .expect("Should contain valid utf-8"),
        )
    }
}


impl Display for LibfsntfsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut buffer = vec![0; 1024];
        let mut _code = self.code.clone();

        let fmt = unsafe {
            libfsntfs_error_sprint(&mut _code as *mut _, buffer.as_mut_ptr(), buffer.len())
        };

        f.write_str(
            &String::from_utf8(buffer.into_iter().map(|c| c as u8).collect())
                .expect("Should contain valid utf-8"),
        )
    }
}
