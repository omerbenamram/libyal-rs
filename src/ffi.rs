use std::mem;

pub trait AsTypeRef {
    type Ref;

    fn as_type_ref(&mut self) -> Self::Ref;
}

#[macro_export]
macro_rules! declare_ffi_type {
    (
        $(#[$doc:meta])*
        $ty:ident, $raw:ident
    ) => {
        $(#[$doc])*
        pub struct $ty($raw);
    }
}

#[macro_export]
macro_rules! impl_ffi_type {
    ($ty:ident, $ty_ref:ident) => {
        impl $crate::ffi::AsTypeRef for $ty {
            type Ref = $ty_ref;

            #[inline]
            fn as_type_ref(&self) -> Self::Ref {
                self.0 as *const _ as *mut _
            }
        }

        impl $ty {
            pub fn wrap_ptr(ptr: $ty_ref) -> $ty {
                $ty(ptr)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ffi_dtor {
    ($ty:ident, $dtor:ident) => {
        impl Drop for $ty {
            fn drop(&mut self) {
                use crate::ffi::AsTypeRef;
                use log::trace;

                let mut error = ptr::null_mut();

                trace!("Calling `{}`", stringify!($dtor));

                unsafe {
                    $dtor(&mut self.as_type_ref(), &mut error);
                }

                debug_assert!(error.is_null(), "`{}` failed!", stringify!($dtor));
            }
        }
    };
}
