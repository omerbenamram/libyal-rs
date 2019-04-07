use libyal_rs_common::ffi::AsTypeRef;
use crate::ffi_error::{LibfsntfsError, LibfsntfsErrorRef};
use libfsntfs_sys::*;
use failure::Fail;
use std::convert::TryFrom;
use std::ffi::{c_void, FromBytesWithNulError, NulError};
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Failed to convert date {}", _0)]
    FailedToConvertDate(#[cause] chrono::ParseError),
    #[fail(display = "AttributeType has no variant {}", _0)]
    UnknownAttributeEnumVariant(u32),
    #[fail(display = "String is invalid UTF-8: {}", _0)]
    StringContainsInvalidUTF8(#[cause] FromUtf8Error),
    #[fail(display = "String is invalid UTF-8: {}", _0)]
    FailedToConvertFromBytes(#[cause] FromBytesWithNulError),
    #[fail(display = "String contains NUL where is it not allowed: {}", _0)]
    StringContainsNul(#[cause] NulError),
    #[fail(display = "An FFI error has occurred: {}", _0)]
    FFI(String),
    #[fail(display = "An unexpected error has occurred: {}", _0)]
    Other(String),
}
