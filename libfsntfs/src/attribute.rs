use crate::error::Error;
use crate::ffi_error::{LibfsntfsErrorRef, LibfsntfsErrorRefMut};
use crate::file_entry::FileEntry;
use chrono::{Date, DateTime, NaiveDateTime, Utc};
use libfsntfs_sys::size64_t;
use libyal_rs_common::ffi::AsTypeRef;
use log::error;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::{fmt, ptr};

#[repr(C)]
pub struct __Attribute(isize);

pub type AttributeRefMut = *mut __Attribute;
pub type AttributeRef = *const __Attribute;

#[repr(C)]
pub struct Attribute<'a>(AttributeRefMut, &'a FileEntry<'a>);

impl<'a> AsTypeRef for Attribute<'a> {
    type Ref = AttributeRef;
    type RefMut = AttributeRefMut;

    #[inline]
    fn as_type_ref(&self) -> Self::Ref {
        // https://users.rust-lang.org/t/is-it-ub-to-convert-t-to-mut-t/16238/4
        self.0 as *const _
    }

    #[inline]
    fn as_type_ref_mut(&mut self) -> Self::RefMut {
        self.0 as *mut _
    }

    #[inline]
    fn as_raw(&mut self) -> *mut Self::RefMut {
        &mut self.0 as *mut _
    }
}

impl<'a> Attribute<'a> {
    pub fn wrap_ptr(file_entry: &'a FileEntry<'a>, ptr: AttributeRefMut) -> Self {
        Attribute(ptr, file_entry)
    }
}

impl<'a> Drop for Attribute<'a> {
    fn drop(&mut self) {
        use libyal_rs_common::ffi::AsTypeRef;
        use log::trace;

        let mut error = ptr::null_mut();

        trace!("Calling `libfsntfs_attribute_free`");

        unsafe {
            libfsntfs_attribute_free(self.as_raw(), &mut error);
        }

        debug_assert!(error.is_null(), "`libfsntfs_attribute_free` failed!");
    }
}

impl<'a> Debug for Attribute<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Attribute")
            .field("Name", &self.get_name().unwrap_or("".to_string()))
            .field(
                "Type",
                &self
                    .get_type()
                    .and_then(|a| Ok(format!("{:?}", a)))
                    .unwrap_or_else(|_| "".to_string()),
            )
            .finish()
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
#[repr(C)]
pub enum AttributeType {
    Unused = 0,
    StandardInformation = 16,
    AttributeList = 32,
    FileName = 48,
    ObjectIdentifier = 64,
    SecurityDescriptor = 80,
    VolumeName = 96,
    VolumeInformation = 112,
    Data = 128,
    IndexRoot = 144,
    IndexAllocation = 160,
    Bitmap = 176,
    ReparsePoint = 192,
    ExtendedInformation = 208,
    Extended = 224,
    PropertySet = 240,
    LoggedUtilityStream = 256,
    EndOfAttributes = 4_294_967_295u32 as isize,
}

impl TryFrom<u32> for AttributeType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AttributeType::Unused),
            16 => Ok(AttributeType::StandardInformation),
            32 => Ok(AttributeType::AttributeList),
            48 => Ok(AttributeType::FileName),
            64 => Ok(AttributeType::ObjectIdentifier),
            80 => Ok(AttributeType::SecurityDescriptor),
            96 => Ok(AttributeType::VolumeName),
            112 => Ok(AttributeType::VolumeInformation),
            128 => Ok(AttributeType::Data),
            144 => Ok(AttributeType::IndexRoot),
            160 => Ok(AttributeType::IndexAllocation),
            176 => Ok(AttributeType::Bitmap),
            192 => Ok(AttributeType::ReparsePoint),
            208 => Ok(AttributeType::ExtendedInformation),
            224 => Ok(AttributeType::Extended),
            240 => Ok(AttributeType::PropertySet),
            256 => Ok(AttributeType::LoggedUtilityStream),
            4_294_967_295 => Ok(AttributeType::EndOfAttributes),
            _ => Err(Error::UnknownAttributeEnumVariant(value)),
        }
    }
}

extern "C" {
    pub fn libfsntfs_attribute_free(
        attribute: *mut AttributeRefMut,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_type(
        attribute: AttributeRef,
        type_: *mut u32,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_data_flags(
        attribute: AttributeRef,
        data_flags: *mut u16,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf8_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf8_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf16_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf16_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_data_vcn_range(
        attribute: AttributeRef,
        data_first_vcn: *mut u64,
        data_last_vcn: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_file_reference(
        attribute: AttributeRef,
        mft_entry_index: *mut u64,
        sequence_number: *mut u16,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_data_size(
        attribute: AttributeRef,
        data_size: *mut size64_t,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_parent_file_reference(
        attribute: AttributeRef,
        parent_file_reference: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_creation_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_access_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_entry_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_file_attribute_flags(
        attribute: AttributeRef,
        file_attribute_flags: *mut u32,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf8_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf8_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf16_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf16_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_droid_file_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_birth_droid_volume_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_birth_droid_file_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_birth_droid_domain_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_tag(
        attribute: AttributeRef,
        tag: *mut u32,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_substitute_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_substitute_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_substitute_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_substitute_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_print_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_print_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_print_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_print_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_security_descriptor_attribute_get_security_descriptor_size(
        attribute: AttributeRef,
        data_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_security_descriptor_attribute_get_security_descriptor(
        attribute: AttributeRef,
        data: *mut u8,
        data_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_creation_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_access_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_entry_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_file_attribute_flags(
        attribute: AttributeRef,
        file_attribute_flags: *mut u32,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_owner_identifier(
        attribute: AttributeRef,
        owner_identifier: *mut u32,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_security_descriptor_identifier(
        attribute: AttributeRef,
        security_descriptor_identifier: *mut u32,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_update_sequence_number(
        attribute: AttributeRef,
        update_sequence_number: *mut u64,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_volume_information_attribute_get_version(
        attribute: AttributeRef,
        major_version: *mut u8,
        minor_version: *mut u8,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_volume_information_attribute_get_flags(
        attribute: AttributeRef,
        flags: *mut u16,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf8_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf8_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf16_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf16_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRefMut,
    ) -> c_int;
}

#[derive(Debug, Clone)]
pub enum AttributeWithInformation {
    StandardInformation(StandardInformation),
    FileName(FileName),
    SecurityDescriptor(SecurityDescriptor),
    VolumeName(String),
    VolumeInformation(VolumeInformation),

    AttributeList(AttributeList),
    ObjectIdentifier(ObjectIdentifier),
    Data(Data),
    IndexRoot(IndexRoot),
    IndexAllocation(IndexAllocation),
    Bitmap(Bitmap),
    ReparsePoint(ReparsePoint),
    ExtendedInformation(ExtendedInformation),
    Extended(Extended),
    PropertySet(PropertySet),
    LoggedUtilityStream(LoggedUtilityStream),
    EndOfAttributes(EndOfAttributes),
}

#[derive(Debug, Clone)]
pub struct StandardInformation {
    pub creation_time: Option<DateTime<Utc>>,
    pub modification_time: Option<DateTime<Utc>>,
    pub access_time: Option<DateTime<Utc>>,
    pub entry_modification_time: Option<DateTime<Utc>>,
    pub file_attribute_flags: u32,
    pub owner_identifier: u32,
    pub security_descriptor_identifier: u32,
    pub update_sequence_number: u32,
}

#[derive(Debug, Clone)]
pub struct FileName {
    pub name: String,
    pub parent_file_reference: u64,
    pub creation_time: Option<DateTime<Utc>>,
    pub modification_time: Option<DateTime<Utc>>,
    pub access_time: Option<DateTime<Utc>>,
    pub entry_modification_time: Option<DateTime<Utc>>,
    pub file_attribute_flags: u32,
}

#[derive(Debug, Clone)]
pub struct Data {
    // TOOD: parse flags
    pub flags: u32,
    pub vcn_range_first: u64,
    pub vcn_range_last: u64,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct VolumeInformation {
    pub version: u32,
    // TODO: parse flags
    pub flags: u16,
}

#[derive(Debug, Clone)]
pub struct SecurityDescriptor(Vec<u8>);

#[derive(Debug, Clone)]
pub struct AttributeList {}
#[derive(Debug, Clone)]
pub struct ObjectIdentifier {}
#[derive(Debug, Clone)]
pub struct IndexRoot {}
#[derive(Debug, Clone)]
pub struct IndexAllocation {}
#[derive(Debug, Clone)]
pub struct Bitmap {}
#[derive(Debug, Clone)]
pub struct ReparsePoint {}
#[derive(Debug, Clone)]
pub struct ExtendedInformation {}
#[derive(Debug, Clone)]
pub struct Extended {}
#[derive(Debug, Clone)]
pub struct PropertySet {}
#[derive(Debug, Clone)]
pub struct LoggedUtilityStream {}
#[derive(Debug, Clone)]
pub struct EndOfAttributes {}

impl<'a> Attribute<'a> {
    pub fn get_name(&self) -> Result<String, Error> {
        get_sized_utf8_string!(
            self,
            libfsntfs_attribute_get_utf8_name_size,
            libfsntfs_attribute_get_utf8_name
        )
    }

    pub fn get_data(&self) -> Result<AttributeWithInformation, Error> {
        match self.get_type()? {
            AttributeType::VolumeName => {
                let volume_name = get_sized_utf8_string!(
                    self,
                    libfsntfs_volume_name_attribute_get_utf8_name_size,
                    libfsntfs_volume_name_attribute_get_utf8_name
                )?;

                Ok(AttributeWithInformation::VolumeName(volume_name))
            }
            AttributeType::FileName => {
                let name = get_sized_utf8_string!(
                    self,
                    libfsntfs_file_name_attribute_get_utf8_name_size,
                    libfsntfs_file_name_attribute_get_utf8_name
                )?;

                let creation_time =
                    get_date_field!(self, libfsntfs_file_name_attribute_get_creation_time)?;
                let modification_time =
                    get_date_field!(self, libfsntfs_file_name_attribute_get_modification_time)?;
                let access_time =
                    get_date_field!(self, libfsntfs_file_name_attribute_get_access_time)?;
                let entry_modification_time = get_date_field!(
                    self,
                    libfsntfs_file_name_attribute_get_entry_modification_time
                )?;

                let parent_file_reference = get_u64_field!(
                    self,
                    libfsntfs_file_name_attribute_get_parent_file_reference
                )?;

                Ok(AttributeWithInformation::FileName(FileName {
                    name,
                    parent_file_reference,
                    creation_time,
                    modification_time,
                    access_time,
                    entry_modification_time,
                    file_attribute_flags: 0,
                }))
            }
            AttributeType::StandardInformation => {
                let creation_time = get_date_field!(
                    self,
                    libfsntfs_standard_information_attribute_get_creation_time
                )?;
                let modification_time = get_date_field!(
                    self,
                    libfsntfs_standard_information_attribute_get_modification_time
                )?;
                let access_time = get_date_field!(
                    self,
                    libfsntfs_standard_information_attribute_get_access_time
                )?;
                let entry_modification_time = get_date_field!(
                    self,
                    libfsntfs_standard_information_attribute_get_entry_modification_time
                )?;

                Ok(AttributeWithInformation::StandardInformation(
                    StandardInformation {
                        creation_time,
                        modification_time,
                        access_time,
                        entry_modification_time,
                        file_attribute_flags: 0,
                        owner_identifier: 0,
                        security_descriptor_identifier: 0,
                        update_sequence_number: 0,
                    },
                ))
            }
            AttributeType::Data => Ok(AttributeWithInformation::Data(Data {
                flags: 0,
                vcn_range_first: 0,
                vcn_range_last: 0,
                size: 0,
            })),

            AttributeType::SecurityDescriptor => {
                let descriptor = get_sized_bytes!(
                    self,
                    libfsntfs_security_descriptor_attribute_get_security_descriptor_size,
                    libfsntfs_security_descriptor_attribute_get_security_descriptor
                )?;

                Ok(AttributeWithInformation::SecurityDescriptor(
                    SecurityDescriptor(descriptor),
                ))
            }
            _ => Err(Error::Other(format!(
                "Unimplemented data type: {:?}",
                self.get_type().unwrap()
            ))),
        }
    }

    pub fn get_type(&self) -> Result<AttributeType, Error> {
        let mut type_as_num = 0_u32;
        let mut error = ptr::null_mut();

        if unsafe { libfsntfs_attribute_get_type(self.as_type_ref(), &mut type_as_num, &mut error) }
            != 1
        {
            Err(Error::try_from(error)?)
        } else {
            Ok(AttributeType::try_from(type_as_num)?)
        }
    }
}
