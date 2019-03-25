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
            fn as_type_ref(&mut self) -> Self::Ref {
                self.0
           }
        }

        impl $ty {
            pub fn wrap_ptr(ptr: $ty_ref) -> $ty {
                 $ty(ptr)
            }
        }
    }
}