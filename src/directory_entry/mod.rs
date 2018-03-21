use both_endian::{BothEndian16, BothEndian32};
use datetime::DateTime;

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

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DirectoryEntryHeader {
    pub length: u8,
    pub extended_attribute_record_length: u8,
    pub extent_loc: BothEndian32,
    pub extent_length: BothEndian32,
    pub time: DateTime,
    pub file_flags: FileFlags,
    pub file_unit_size: u8,
    pub interleave_gap_size: u8,
    pub volume_sequence_number: BothEndian16,
    pub file_identifier_len: u8,
}

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

assert_eq_size!(directory_hdr_size_eq; DirectoryEntryHeader, [u8; 33]);
