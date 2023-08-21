// SPDX-License-Identifier: (MIT OR Apache-2.0)

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::{map, map_res, value};
use nom::number::complete::*;
use nom::sequence::tuple;
use nom::IResult;
use std::str;
use time::OffsetDateTime;

use super::both_endian::{both_endian16, both_endian32};
use super::date_time::date_time_ascii;
use super::directory_entry::{directory_entry, DirectoryEntryHeader};
use crate::Result;

#[derive(Clone, Debug)]
pub struct VolumeDescriptorTable {
    pub system_identifier: String,
    pub volume_identifier: String,
    pub character_encoding: CharacterEncoding,
    pub volume_space_size: u32,
    pub volume_set_size: u16,
    pub volume_sequence_number: u16,
    pub logical_block_size: u16,

    pub path_table_size: u32,
    pub path_table_loc: u32,
    pub optional_path_table_loc: u32,

    pub root_directory_entry: DirectoryEntryHeader,
    pub root_directory_entry_identifier: String,

    pub volume_set_identifier: String,
    pub publisher_identifier: String,
    pub data_preparer_identifier: String,
    pub application_identifier: String,
    pub copyright_file_identifier: String,
    pub abstract_file_identifier: String,
    pub bibliographic_file_identifier: String,

    pub creation_time: OffsetDateTime,
    pub modification_time: OffsetDateTime,
    pub expiration_time: OffsetDateTime,
    pub effective_time: OffsetDateTime,

    pub file_structure_version: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharacterEncoding {
    Iso9660,
    Ucs2Level1,
    Ucs2Level2,
    Ucs2Level3,
}

impl CharacterEncoding {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        // The field is 32 bytes long, and per §8.5.6 ECMA says there can be multiple
        // encodings listed.  But. Really.  For now let's just check for the standard
        // ISO 9660 encoding or UCS-2.

        // Per ECMA 35 / ISO 2022 §13.2.2:
        // I byte 0x25 (02/05) = Designate Other Coding System
        //
        // Per ECMA 35 / ISO 2022 §15.4.2:
        // DOCS with I byte 0x2F (02/15) shall mean it's not really DOCS and we should
        // use the registry as a reference.
        //
        // ISO Registry shows:
        // #162 UCS-2 Level 1 F byte is 0x40 (04/00)
        // #175 UCS-2 Level 2 F byte is 0x43 (04/03)
        // #175 UCS-2 Level 3 F byte is 0x45 (04/05)
        let orig_len = bytes.len();

        let (bytes, encoding) = alt((
            value(CharacterEncoding::Ucs2Level1, tag(&[0x25, 0x2F, 0x40])),
            value(CharacterEncoding::Ucs2Level2, tag(&[0x25, 0x2F, 0x43])),
            value(CharacterEncoding::Ucs2Level3, tag(&[0x25, 0x2F, 0x45])),
            value(CharacterEncoding::Iso9660, tag(&[0_u8; 32])),
        ))(bytes)?;

        let bytes = match orig_len - bytes.len() {
            len if len < 32 => {
                take(32 - len)(bytes)?.0
            },
            _ => bytes
        };

        Ok((bytes, encoding))
    }
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) enum VolumeDescriptor {
    Primary(VolumeDescriptorTable),
    Supplementary(VolumeDescriptorTable),
    BootRecord {
        boot_system_identifier: String,
        boot_identifier: String,
        data: Vec<u8>,
    },
    VolumeDescriptorSetTerminator,
}

impl VolumeDescriptor {
    pub fn parse(bytes: &[u8]) -> Result<Option<VolumeDescriptor>> {
        Ok(volume_descriptor(bytes)?.1)
    }
}

fn take_string_trim(count: usize) -> impl Fn(&[u8]) -> IResult<&[u8], String> {
    move |i: &[u8]| {
        map(
            map(map_res(take(count), str::from_utf8), str::trim_end),
            str::to_string,
        )(i)
    }
}

fn boot_record(i: &[u8]) -> IResult<&[u8], VolumeDescriptor> {
    let (i, (boot_system_identifier, boot_identifier, data)) = tuple((
        take_string_trim(32usize),
        take_string_trim(32usize),
        take(1977usize),
    ))(i)?;
    Ok((
        i,
        VolumeDescriptor::BootRecord {
            boot_system_identifier,
            boot_identifier,
            data: data.to_vec(),
        },
    ))
}

fn volume_descriptor(i: &[u8]) -> IResult<&[u8], Option<VolumeDescriptor>> {
    let (i, type_code) = le_u8(i)?;
    let (i, _) = tag("CD001\u{1}")(i)?;
    match type_code {
        0 => map(boot_record, Some)(i),
        1 => map(primary_descriptor, Some)(i),
        2 => map(supplementary_descriptor, Some)(i),
        //3 => map!(volume_partition_descriptor, Some)(i),
        255 => Ok((i, Some(VolumeDescriptor::VolumeDescriptorSetTerminator))),
        _ => Ok((i, None)),
    }
}

fn descriptor_table(i: &[u8]) -> IResult<&[u8], VolumeDescriptorTable> {
    let (i, _) = take(1usize)(i)?; // padding
    let (i, system_identifier) = take_string_trim(32usize)(i)?;
    let (i, volume_identifier) = take_string_trim(32usize)(i)?;
    let (i, _) = take(8usize)(i)?; // padding
    let (i, volume_space_size) = both_endian32(i)?;
    let (i, character_encoding) = CharacterEncoding::parse(i)?;
    let (i, volume_set_size) = both_endian16(i)?;
    let (i, volume_sequence_number) = both_endian16(i)?;
    let (i, logical_block_size) = both_endian16(i)?;

    let (i, path_table_size) = both_endian32(i)?;
    let (i, path_table_loc) = le_u32(i)?;
    let (i, optional_path_table_loc) = le_u32(i)?;
    let (i, _) = take(4usize)(i)?; // path_table_loc_be
    let (i, _) = take(4usize)(i)?; // optional_path_table_loc_be

    let (i, root_directory_entry) = directory_entry(i, character_encoding)?;

    let (i, volume_set_identifier) = take_string_trim(128)(i)?;
    let (i, publisher_identifier) = take_string_trim(128)(i)?;
    let (i, data_preparer_identifier) = take_string_trim(128)(i)?;
    let (i, application_identifier) = take_string_trim(128)(i)?;
    let (i, copyright_file_identifier) = take_string_trim(38)(i)?;
    let (i, abstract_file_identifier) = take_string_trim(36)(i)?;
    let (i, bibliographic_file_identifier) = take_string_trim(37)(i)?;

    let (i, creation_time) = date_time_ascii(i)?;
    let (i, modification_time) = date_time_ascii(i)?;
    let (i, expiration_time) = date_time_ascii(i)?;
    let (i, effective_time) = date_time_ascii(i)?;

    let (i, file_structure_version) = le_u8(i)?;

    Ok((
        i,
        VolumeDescriptorTable {
            system_identifier,
            volume_identifier,
            character_encoding,
            volume_space_size,
            volume_set_size,
            volume_sequence_number,
            logical_block_size,

            path_table_size,
            path_table_loc,
            optional_path_table_loc,

            root_directory_entry: root_directory_entry.0,
            root_directory_entry_identifier: root_directory_entry.1,

            volume_set_identifier,
            publisher_identifier,
            data_preparer_identifier,
            application_identifier,
            copyright_file_identifier,
            abstract_file_identifier,
            bibliographic_file_identifier,

            creation_time,
            modification_time,
            expiration_time,
            effective_time,

            file_structure_version,
        },
    ))
}

fn supplementary_descriptor<'a>(i: &'a [u8]) -> IResult<&'a [u8], VolumeDescriptor> {
    map(descriptor_table, VolumeDescriptor::Supplementary)(i)
}

 fn primary_descriptor(i: &[u8]) -> IResult<&[u8], VolumeDescriptor> {
    map(descriptor_table, VolumeDescriptor::Primary)(i)
}
