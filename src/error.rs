use crate::ffi::AsFFIPtr;
use crate::libfsntfs::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::ffi::c_void;
use failure::Fail;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display="An FFI error has occurred: {}", _0)]
    FFI(String)
}

impl Error {
    pub fn ffi(e: LibfsntfsErrorRef) -> Self {
        Error::FFI(format!("{}", e))
    }
}
