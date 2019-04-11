use failure::Fail;

use std::ffi::{FromBytesWithNulError, NulError};
use std::string::FromUtf8Error;
use std::io;

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
    #[fail(display = "Failed to open file: {}", _0)]
    FailedToOpenFile(#[cause] io::Error),
    #[fail(display = "An FFI error has occurred: {}", _0)]
    FFI(String),
    #[fail(display = "An unexpected error has occurred: {}", _0)]
    Other(String),
}
