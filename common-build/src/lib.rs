#[cfg(not(target_os = "windows"))]
mod posix;

#[cfg(not(target_os = "windows"))]
pub use crate::posix::build_lib;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use crate::windows::build_lib;