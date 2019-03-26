use crate::error::Error;
use crate::ffi::AsTypeRef;
use crate::ffi_error::LibfsntfsErrorRef;
use crate::libfsntfs::size64_t;
use crate::utils::str_from_u8_nul_utf8_unchecked;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::os::raw::c_int;
use std::{fmt, ptr};

#[repr(C)]
pub struct __Attribute(isize);

pub type AttributeRef = *mut __Attribute;

declare_ffi_type!(Attribute, AttributeRef);
impl_ffi_type!(Attribute, AttributeRef);
impl_ffi_dtor!(Attribute, libfsntfs_attribute_free);

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum AttributeType {
    AttributeTypeUnused = 0,
    AttributeTypeStandardInformation = 16,
    AttributeTypeAttributeList = 32,
    AttributeTypeFileName = 48,
    AttributeTypeObjectIdentifier = 64,
    AttributeTypeSecurityDescriptor = 80,
    AttributeTypeVolumeName = 96,
    AttributeTypeVolumeInformation = 112,
    AttributeTypeData = 128,
    AttributeTypeIndexRoot = 144,
    AttributeTypeIndexAllocation = 160,
    AttributeTypeBitmap = 176,
    AttributeTypeReparsePoint = 192,
    AttributeTypeExtendedInformation = 208,
    AttributeTypeExtended = 224,
    AttributeTypePropertySet = 240,
    AttributeTypeLoggedUtilityStream = 256,
    AttributeTypeEndOfAttributes = 4294967295,
}

impl TryFrom<u32> for AttributeType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AttributeType::AttributeTypeUnused),
            16 => Ok(AttributeType::AttributeTypeStandardInformation),
            32 => Ok(AttributeType::AttributeTypeAttributeList),
            48 => Ok(AttributeType::AttributeTypeFileName),
            64 => Ok(AttributeType::AttributeTypeObjectIdentifier),
            80 => Ok(AttributeType::AttributeTypeSecurityDescriptor),
            96 => Ok(AttributeType::AttributeTypeVolumeName),
            112 => Ok(AttributeType::AttributeTypeVolumeInformation),
            128 => Ok(AttributeType::AttributeTypeData),
            144 => Ok(AttributeType::AttributeTypeIndexRoot),
            160 => Ok(AttributeType::AttributeTypeIndexAllocation),
            176 => Ok(AttributeType::AttributeTypeBitmap),
            192 => Ok(AttributeType::AttributeTypeReparsePoint),
            208 => Ok(AttributeType::AttributeTypeExtendedInformation),
            224 => Ok(AttributeType::AttributeTypeExtended),
            240 => Ok(AttributeType::AttributeTypePropertySet),
            256 => Ok(AttributeType::AttributeTypeLoggedUtilityStream),
            4294967295 => Ok(AttributeType::AttributeTypeEndOfAttributes),
            _ => Err(Error::UnknownAttributeEnumVariant(value)),
        }
    }
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Attribute")
            .field("Name", &self.get_name().unwrap_or("".to_string()))
            .field(
                "Type",
                &self
                    .get_type()
                    .and_then(|a| Ok(format!("{:?}", a)))
                    .unwrap_or("".to_string()),
            )
            .finish()
    }
}

extern "C" {
    pub fn libfsntfs_attribute_free(
        attribute: *mut AttributeRef,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_type(
        attribute: AttributeRef,
        type_: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_data_flags(
        attribute: AttributeRef,
        data_flags: *mut u16,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf8_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf8_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf16_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_utf16_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_data_vcn_range(
        attribute: AttributeRef,
        data_first_vcn: *mut u64,
        data_last_vcn: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_file_reference(
        attribute: AttributeRef,
        mft_entry_index: *mut u64,
        sequence_number: *mut u16,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_attribute_get_data_size(
        attribute: AttributeRef,
        data_size: *mut size64_t,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_parent_file_reference(
        attribute: AttributeRef,
        parent_file_reference: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_creation_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_access_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_entry_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_file_attribute_flags(
        attribute: AttributeRef,
        file_attribute_flags: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf8_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf8_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf16_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_file_name_attribute_get_utf16_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_droid_file_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_birth_droid_volume_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_birth_droid_file_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_object_identifier_attribute_get_birth_droid_domain_identifier(
        attribute: AttributeRef,
        guid: *mut u8,
        size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_tag(
        attribute: AttributeRef,
        tag: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_substitute_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_substitute_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_substitute_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_substitute_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_print_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf8_print_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_print_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_reparse_point_attribute_get_utf16_print_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_security_descriptor_attribute_get_security_descriptor_size(
        attribute: AttributeRef,
        data_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_security_descriptor_attribute_get_security_descriptor(
        attribute: AttributeRef,
        data: *mut u8,
        data_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_creation_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_access_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_entry_modification_time(
        attribute: AttributeRef,
        filetime: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_file_attribute_flags(
        attribute: AttributeRef,
        file_attribute_flags: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_owner_identifier(
        attribute: AttributeRef,
        owner_identifier: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_security_descriptor_identifier(
        attribute: AttributeRef,
        security_descriptor_identifier: *mut u32,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_standard_information_attribute_get_update_sequence_number(
        attribute: AttributeRef,
        update_sequence_number: *mut u64,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_information_attribute_get_version(
        attribute: AttributeRef,
        major_version: *mut u8,
        minor_version: *mut u8,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_information_attribute_get_flags(
        attribute: AttributeRef,
        flags: *mut u16,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf8_name_size(
        attribute: AttributeRef,
        utf8_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf8_name(
        attribute: AttributeRef,
        utf8_name: *mut u8,
        utf8_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf16_name_size(
        attribute: AttributeRef,
        utf16_name_size: *mut usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
    pub fn libfsntfs_volume_name_attribute_get_utf16_name(
        attribute: AttributeRef,
        utf16_name: *mut u16,
        utf16_name_size: usize,
        error: *mut LibfsntfsErrorRef,
    ) -> c_int;
}

impl Attribute {
    pub fn get_name(&self) -> Result<String, Error> {
        get_sized_utf8_string!(
            self,
            libfsntfs_attribute_get_utf8_name_size,
            libfsntfs_attribute_get_utf8_name
        )
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
