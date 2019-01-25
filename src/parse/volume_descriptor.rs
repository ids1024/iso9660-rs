#![allow(dead_code)]

use nom::{le_u8, le_u32};
use time::Tm;

use crate::ISOError;
use super::directory_entry::{DirectoryEntryHeader, directory_entry};
use super::both_endian::{both_endian16, both_endian32};
use super::date_time::date_time_ascii;

#[derive(Clone, Debug)]
pub(crate) enum VolumeDescriptor {
    Primary {
        system_identifier: String,
        volume_identifier: String,
        volume_space_size: u32,
        volume_set_size: u16,
        volume_sequence_number: u16,
        logical_block_size: u16,

        path_table_size: u32,
        path_table_loc: u32,
        optional_path_table_loc: u32,

        root_directory_entry: DirectoryEntryHeader,
        root_directory_entry_identifier: String,

        volume_set_identifier: String,
        publisher_identifier: String,
        data_preparer_identifier: String,
        application_identifier: String,
        copyright_file_identifier: String,
        abstract_file_identifier: String,
        bibliographic_file_identifier: String,

        creation_time: Tm,
        modification_time: Tm,
        expiration_time: Tm,
        effective_time: Tm,

        file_structure_version: u8,
    },
    BootRecord {
        boot_system_identifier: String,
        boot_identifier: String,
        data: Vec<u8> 
    },
    VolumeDescriptorSetTerminator
}

impl VolumeDescriptor {
    pub fn parse(bytes: &[u8]) -> Result<Option<VolumeDescriptor>, ISOError> {
        Ok(volume_descriptor(bytes)?.1)
    }
}

named!(boot_record<&[u8], VolumeDescriptor>, do_parse!(
    boot_system_identifier: take_str!(32) >>
    boot_identifier:        take_str!(32) >>
    data:                   take!(1977)   >>
    (VolumeDescriptor::BootRecord {
        boot_system_identifier: boot_system_identifier.trim_end().to_string(),
        boot_identifier: boot_identifier.trim_end().to_string(),
        data: data.to_vec()
    })
));

named!(volume_descriptor<&[u8], Option<VolumeDescriptor>>, do_parse!(
    type_code: le_u8         >>
               tag!("CD001") >>
               tag!("\u{1}") >>

    res: switch!(value!(type_code),
        0 => map!(call!(boot_record), Some)        |
        1 => map!(call!(primary_descriptor), Some) |
        //2 => map!(call!(supplementary_volume_descriptor), Some) |
        //3 => map!(call!(volume_partition_descriptor), Some)        |
        255 => value!(Some(VolumeDescriptor::VolumeDescriptorSetTerminator)) |
        _ => value!(None)
    ) >>
    (res)
));

named!(primary_descriptor<&[u8], VolumeDescriptor>, do_parse!(
    take!(1) >> // padding
    system_identifier: take_str!(32) >>
    volume_identifier: take_str!(32) >>
    take!(8) >> // padding
    volume_space_size: both_endian32 >>
    take!(32) >> // padding
    volume_set_size: both_endian16 >>
    volume_sequence_number: both_endian16 >>
    logical_block_size: both_endian16 >>

    path_table_size: both_endian32 >>
    path_table_loc: le_u32 >>
    optional_path_table_loc: le_u32 >>
    take!(4) >> // path_table_loc_be
    take!(4) >> // optional_path_table_loc_be

    root_directory_entry: length_value!(value!(34), directory_entry) >>

    volume_set_identifier: take_str!(128) >>
    publisher_identifier: take_str!(128) >>
    data_preparer_identifier: take_str!(128) >>
    application_identifier: take_str!(128) >>
    copyright_file_identifier: take_str!(38) >>
    abstract_file_identifier: take_str!(36) >>
    bibliographic_file_identifier: take_str!(37) >>

    creation_time: date_time_ascii >>
    modification_time: date_time_ascii >>
    expiration_time: date_time_ascii >>
    effective_time: date_time_ascii >>

    file_structure_version: le_u8 >>

    (VolumeDescriptor::Primary {
        system_identifier: system_identifier.trim_end().to_string(),
        volume_identifier: volume_identifier.trim_end().to_string(),
        volume_space_size,
        volume_set_size,
        volume_sequence_number,
        logical_block_size,

        path_table_size,
        path_table_loc,
        optional_path_table_loc,

        root_directory_entry: root_directory_entry.0,
        root_directory_entry_identifier: root_directory_entry.1,

        volume_set_identifier: volume_set_identifier.trim_end().to_string(),
        publisher_identifier: publisher_identifier.trim_end().to_string(),
        data_preparer_identifier: data_preparer_identifier.trim_end().to_string(),
        application_identifier: application_identifier.trim_end().to_string(),
        copyright_file_identifier: copyright_file_identifier.trim_end().to_string(),
        abstract_file_identifier: abstract_file_identifier.trim_end().to_string(),
        bibliographic_file_identifier: bibliographic_file_identifier.trim_end().to_string(),

        creation_time,
        modification_time,
        expiration_time,
        effective_time,

        file_structure_version,
    })
));
