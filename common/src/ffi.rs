use std::mem;

pub trait AsTypeRef {
    type Ref;
    type RefMut;

    /// should return *const inner
    fn as_type_ref(&self) -> Self::Ref;

    /// should return *mut inner
    fn as_type_ref_mut(&mut self) -> Self::RefMut;

    /// Used for d'tors - should return *mut *mut inner
    fn as_raw(&mut self) -> *mut Self::RefMut;
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
macro_rules! get_sized_bytes {
    ($self: ident, $get_size: ident, $get_string: ident) => {{
        let mut size = 0_usize;
        let mut error = ptr::null_mut();

        if unsafe { $get_size($self.as_type_ref(), &mut size, &mut error) } != 1 {
            return Err(Error::try_from(error)?);
        };

        if size == 0 {
            Ok(Vec::<u8>::new())
        } else {
            let mut data = vec![0; size];
            let mut error = ptr::null_mut();

            if unsafe {
                $get_string(
                    $self.as_type_ref(),
                    data.as_mut_ptr(),
                    data.len(),
                    &mut error,
                )
            } != 1
            {
                Err(Error::try_from(error)?)
            } else {
                Ok(data)
            }
        }
    }};
}

#[macro_export]
macro_rules! get_date_field {
    ($self: ident, $getter: ident) => {{
        use crate::utils::datetime_from_filetime;
        use chrono::prelude::*;

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
