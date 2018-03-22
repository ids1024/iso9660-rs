use nom::{le_u8, le_u32};
use time::Tm;
use std::str::{self, Utf8Error};

use ::ISOError;
use super::directory_entry::{DirectoryEntryHeader, directory_entry};
use super::both_endian::{both_endian16, both_endian32};
use super::date_time::date_time_ascii;

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
        // XXX unwrap
        let (_, desc) = volume_descriptor(bytes).unwrap();
        Ok(desc)
    }
}

fn identifier_to_string(identifier: &[u8]) -> Result<String, Utf8Error> {
    let end = identifier.iter().rposition(|x| *x != b' ')
                               .map(|x| x + 1)
                               .unwrap_or(0);
    Ok(str::from_utf8(&identifier[..end])?.to_string())
}

named!(boot_record<&[u8], VolumeDescriptor>, do_parse!(
    boot_system_identifier: take!(32)   >>
    boot_identifier:        take!(32)   >>
    data:                   take!(1977) >>
    (VolumeDescriptor::BootRecord {
        // XXX unwrap
        boot_system_identifier: identifier_to_string(boot_system_identifier).unwrap(),
        boot_identifier: identifier_to_string(boot_identifier).unwrap(),
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
    system_identifier: take!(32) >>
    volume_identifier: take!(32) >>
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

    volume_set_identifier: take!(128) >>
    publisher_identifier: take!(128) >>
    data_preparer_identifier: take!(128) >>
    application_identifier: take!(128) >>
    copyright_file_identifier: take!(38) >>
    abstract_file_identifier: take!(36) >>
    bibliographic_file_identifier: take!(37) >>

    creation_time: date_time_ascii >>
    modification_time: date_time_ascii >>
    expiration_time: date_time_ascii >>
    effective_time: date_time_ascii >>

    file_structure_version: le_u8 >>

    (VolumeDescriptor::Primary {
        // XXX unwrap
        system_identifier: identifier_to_string(system_identifier).unwrap(),
        volume_identifier: identifier_to_string(volume_identifier).unwrap(),
        volume_space_size,
        volume_set_size,
        volume_sequence_number,
        logical_block_size,

        path_table_size,
        path_table_loc,
        optional_path_table_loc,

        root_directory_entry,

        volume_set_identifier: identifier_to_string(volume_set_identifier).unwrap(),
        publisher_identifier: identifier_to_string(publisher_identifier).unwrap(),
        data_preparer_identifier: identifier_to_string(data_preparer_identifier).unwrap(),
        application_identifier: identifier_to_string(application_identifier).unwrap(),
        copyright_file_identifier: identifier_to_string(copyright_file_identifier).unwrap(),
        abstract_file_identifier: identifier_to_string(abstract_file_identifier).unwrap(),
        bibliographic_file_identifier: identifier_to_string(bibliographic_file_identifier).unwrap(),

        creation_time,
        modification_time,
        expiration_time,
        effective_time,

        file_structure_version,
    })
));
