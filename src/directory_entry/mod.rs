use nom::{le_u8, le_u16, le_u32};
use time::Tm;
use std::str;

pub use self::isodirectory::ISODirectory;
pub use self::isofile::ISOFile;

mod isodirectory;
mod isofile;

bitflags! {
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
    pub time: Tm,
    pub file_flags: FileFlags,
    pub file_unit_size: u8,
    pub interleave_gap_size: u8,
    pub volume_sequence_number: u16,
    pub identifier: String,
}

named!(pub both_endian16<&[u8], u16>, do_parse!(
    // Only reading the little endian version, which is first.
    // The Linux kernel does the same, with a comment about some programs
    // generating invalid ISO with incorrect big endian values.
    val: le_u16   >>
         take!(2) >>
    (val)
));

named!(pub both_endian32<&[u8], u32>, do_parse!(
    val: le_u32   >>
         take!(4) >>
    (val)
));

named!(date_time<&[u8], Tm>, do_parse!(
    year:       le_u8 >> // years since 1900
    month:      le_u8 >>
    day:        le_u8 >>
    hour:       le_u8 >>
    minute:     le_u8 >>
    second:     le_u8 >>
    gmt_offset: le_u8 >>
    (Tm {
        tm_year: year as i32,
        tm_mon: month as i32,
        tm_hour: hour as i32,
        tm_min: minute as i32,
        tm_sec: second as i32,
        tm_mday: day as i32,
        tm_wday: -1, // XXX
        tm_yday: -1, // XXX
        tm_nsec: 0,
        tm_isdst: -1,
        tm_utcoff: gmt_offset as i32,
    })

));

named!(pub directory_entry<&[u8], DirectoryEntryHeader>, do_parse!(
    length:                           le_u8                >>
    extended_attribute_record_length: le_u8                >>
    extent_loc:                       both_endian32        >>
    extent_length:                    both_endian32        >>
    time:                             date_time            >>
    file_flags:                       le_u8                >>
    file_unit_size:                   le_u8                >>
    interleave_gap_size:              le_u8                >>
    volume_sequence_number:           both_endian16        >>
    identifier:                       length_bytes!(le_u8) >>
    // After the file identifier, ISO 9660 allows addition space for
    // system use. Ignore that for now.

    (DirectoryEntryHeader {
        length,
        extended_attribute_record_length,
        extent_loc,
        extent_length,
        time,
        file_flags: FileFlags::from_bits_truncate(file_flags),
        file_unit_size,
        interleave_gap_size,
        volume_sequence_number,
        // XXX unwrap
        identifier: str::from_utf8(identifier).unwrap().to_string(),
    })
));

#[derive(Clone, Debug)]
pub enum DirectoryEntry {
    Directory(ISODirectory),
    File(ISOFile)
}

impl DirectoryEntry {
    pub(crate) fn header(&self) -> &DirectoryEntryHeader {
        match *self {
            DirectoryEntry::Directory(ref dir) => &dir.header,
            DirectoryEntry::File(ref file) => &file.header,
        }

    }

    pub fn identifier(&self) -> &str {
        match *self {
            DirectoryEntry::Directory(ref dir) => &dir.identifier,
            DirectoryEntry::File(ref file) => &file.identifier,
        }
    }
}
