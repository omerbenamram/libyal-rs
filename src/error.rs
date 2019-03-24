use crate::libfsntfs::*;
use std::fmt::{self, Display, Formatter};

pub struct LibfsntfsError {
    code: isize,
}

impl Drop for LibfsntfsError {
    fn drop(&mut self) {
        unsafe { libfsntfs_error_free(&mut self.code as *mut _) };
    }
}

impl Display for LibfsntfsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut buffer = vec![0; 1024];
        let mut _code = self.code.clone();

        let fmt = unsafe {
            libfsntfs_error_sprint(&mut _code as *mut _, buffer.as_mut_ptr(), buffer.len())
        };

        f.write_str(&String::from_utf8(buffer)?)
    }
}
