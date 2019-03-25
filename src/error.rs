use std::convert::TryFrom;
use crate::ffi::AsTypeRef;
use crate::libfsntfs::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::ffi::{c_void, NulError};
use failure::Fail;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};
use std::string::FromUtf8Error;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display="String is invalid UTF-8: {}", _0)]
    StringIsInvalidUTF8(#[cause] FromUtf8Error),
    #[fail(display="String contains NUL where is it not allowed: {}", _0)]
    StringContainsNul(#[cause] NulError),
    #[fail(display="An FFI error has occurred: {}", _0)]
    FFI(String),
    #[fail(display="An unexpected error has occurred: {}", _0)]
    Other(String)
}