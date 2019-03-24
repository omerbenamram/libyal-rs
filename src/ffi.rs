pub trait AsFFIPtr {
    type Target;
    fn as_ffi_ptr(&mut self) -> *mut *mut Target;
}


#[macro_export]
macro_rules! impl_as_ffi_ptr {
    ($ffi_handle_type: ident, $s: ident) => {
        impl $crate::ffi::AsFFIPtr for $s {
            type Target = $ffi_handle_type;
            fn as_ffi_ptr(&mut self) -> *mut *mut Target {
                let mut ptr = &mut self.code as *mut _;
                &mut ptr as *mut _
           }
        }
    }
}

impl_as_ffi_ptr!(isize, isize);
impl_as_ffi_ptr!(i32, i32);