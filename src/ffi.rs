use std::mem;

pub trait AsTypeRef {
    type Ref;

    fn as_type_ref(&self) -> Self::Ref;
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
                // https://users.rust-lang.org/t/is-it-ub-to-convert-t-to-mut-t/16238/4
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

#[macro_export]
macro_rules! get_sized_utf8_string {
    ($self: ident, $get_size: ident, $get_string: ident) => {{
        let mut name_size = 0_usize;
        let mut error = ptr::null_mut();

        if unsafe { $get_size($self.as_type_ref(), &mut name_size, &mut error) } != 1 {
            return Err(Error::try_from(error)?);
        };

        if name_size == 0 {
            Ok(String::new())
        } else {
            let mut name = vec![0; name_size];
            let mut error = ptr::null_mut();

            if unsafe {
                $get_string(
                    $self.as_type_ref(),
                    name.as_mut_ptr(),
                    name.len(),
                    &mut error,
                )
            } != 1
            {
                Err(Error::try_from(error)?)
            } else {
                // Discard nul terminator;
                name.pop().expect("name_size was checked to be > 0");
                let s = String::from_utf8(name).map_err(|e| Error::StringContainsInvalidUTF8(e))?;
                Ok(s)
            }
        }
    }};
}

#[macro_export]
macro_rules! get_date_field {
    ($self: ident, $getter: ident) => {{
        use chrono::prelude::*;
        use crate::utils::datetime_from_filetime;

        let mut date = 0_u64;
        let mut error = ptr::null_mut();

        if unsafe { $getter($self.as_type_ref(), &mut date, &mut error) } != 1 {
            Err(Error::try_from(error)?)
        } else {
            let date = if date > 0 {
                Some(datetime_from_filetime(date))
            } else {
                None
            };
            Ok(date)
        }
    }};
}
