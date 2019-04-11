#![feature(seek_convenience)]
#![feature(toowned_clone_into)]

#[macro_use]
extern crate libyal_rs_common;

pub mod handle;
mod io_handle;
pub mod ffi_error;
pub mod error;
//mod libbfio;