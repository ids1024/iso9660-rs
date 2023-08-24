// SPDX-License-Identifier: (MIT OR Apache-2.0)

use time::OffsetDateTime;

use super::both_endian::{both_endian16, both_endian32};
use super::date_time::date_time;
use super::volume_descriptor::CharacterEncoding;
use crate::{Result, ISOError};
use nom::combinator::{map, map_res};
use nom::multi::length_data;
use nom::number::complete::le_u8;
use nom::IResult;
use std::str;

bitflags! {
    #[derive(Clone, Debug)]
    pub struct FileFlags: u8 {
        const EXISTANCE = 1 << 0;
        const DIRECTORY = 1 << 1;
        const ASSOCIATEDFILE = 1 << 2;
        const RECORD = 1 << 3;
        const PROTECTION = 1 << 4;
        // Bits 5 and 6 are reserved; should be zero
        const MULTIEXTENT = 1 << 7;
    }
}

#[derive(Clone, Debug)]
pub struct DirectoryEntryHeader {
    pub length: u8,
    pub extended_attribute_record_length: u8,
    pub extent_loc: u32,
    pub extent_length: u32,
    pub time: OffsetDateTime,
    pub file_flags: FileFlags,
    pub file_unit_size: u8,
    pub interleave_gap_size: u8,
    pub volume_sequence_number: u16,
    pub character_encoding: CharacterEncoding,
}

impl DirectoryEntryHeader {
    pub fn parse(input: &[u8], character_encoding: CharacterEncoding) -> Result<(DirectoryEntryHeader, String)> {
        Ok(directory_entry(input, character_encoding)?.1)
    }
}

pub fn directory_entry(i: &[u8], character_encoding: CharacterEncoding) -> IResult<&[u8], (DirectoryEntryHeader, String)> {
    let (i, length) = le_u8(i)?;
    let (i, extended_attribute_record_length) = le_u8(i)?;
    let (i, extent_loc) = both_endian32(i)?;
    let (i, extent_length) = both_endian32(i)?;
    let (i, time) = date_time(i)?;
    let (i, file_flags) = le_u8(i)?;
    let (i, file_unit_size) = le_u8(i)?;
    let (i, interleave_gap_size) = le_u8(i)?;
    let (i, volume_sequence_number) = both_endian16(i)?;

    let (i, identifier) = match character_encoding {
        CharacterEncoding::Iso9660 => map(map_res(length_data(le_u8), str::from_utf8), str::to_string)(i)?,
        CharacterEncoding::Ucs2Level1 |
        CharacterEncoding::Ucs2Level2 |
        CharacterEncoding::Ucs2Level3 => map_res(
            length_data(le_u8),
            |bytes : &[u8]| {
                // From https://www.unicode.org/faq/utf_bom.html#utf16-11
                // UCS-2 does not describe a data format distinct from UTF-16, because both use
                // exactly the same 16-bit code unit representations. However, UCS-2 does not
                // interpret surrogate code points, and thus cannot be used to conformantly
                // represent supplementary characters.
                if bytes == &[0] {
                    Ok("\u{0}".into())
                } else if bytes == &[1] {
                    Ok("\u{1}".into())
                } else {
                    let (cow, _encoding_used, had_errors) = encoding_rs::UTF_16BE.decode(&bytes);
                    if had_errors {
                        Err(ISOError::Utf16)
                    } else {
                        Ok(cow.to_string())
                    }
                }
            })(i)?,
    };

    // After the file identifier, ISO 9660 allows addition space for
    // system use. Ignore that for now.

    Ok((
        i,
        (
            DirectoryEntryHeader {
                length,
                extended_attribute_record_length,
                extent_loc,
                extent_length,
                time,
                file_flags: FileFlags::from_bits_truncate(file_flags),
                file_unit_size,
                interleave_gap_size,
                volume_sequence_number,
                character_encoding,
            },
            identifier,
        ),
    ))
}
