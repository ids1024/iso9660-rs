use both_endian::{BothEndian16, BothEndian32};
use datetime::DateTime;

pub use self::isodirectory::ISODirectory;
pub use self::isofile::ISOFile;

mod isodirectory;
mod isofile;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DirectoryEntryHeader {
    pub length: u8,
    pub extended_attribute_record_length: u8,
    pub extent_loc: BothEndian32,
    pub extent_length: BothEndian32,
    pub time: DateTime,
    pub file_flags: u8,
    pub file_unit_size: u8,
    pub interleave_gap_size: u8,
    pub volume_sequence_number: BothEndian16,
    pub file_identifier_len: u8,
}

impl DirectoryEntryHeader {
    pub fn is_directory(&self) -> bool {
        self.file_flags & (1 << 1) != 0
    }
}

#[derive(Clone, Debug)]
pub enum DirectoryEntry {
    Directory(ISODirectory),
    File(ISOFile)
}

assert_eq_size!(directory_hdr_size_eq; DirectoryEntryHeader, [u8; 33]);
