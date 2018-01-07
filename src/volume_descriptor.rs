use both_endian::{BothEndian16, BothEndian32};
use datetime::DateTimeAscii;
use super::directory_entry::DirectoryEntryHeader;

// NOTE: If the compiler adds extra padding for some reason, this will
// break (which will be caught by the static assertions. I doubt it would,
// and it works on x86_64 Linux, but it is possible that it could fail with
// certain architectures/ABIs.


#[repr(C)]
pub union VolumeDescriptor {
    pub header: VolumeDescriptorHeader,
    pub primary: PrimaryVolumeDescriptor
}

#[repr(C)]
pub struct VolumeDescriptorHeader {
    pub type_code: u8,
    pub identifier: [u8; 5],
    pub version: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct PrimaryVolumeDescriptor {
    _header: [u8; 7], // Access through VolumeDescriptor.header
    _pad1: u8,
    pub system_identifier: [u8; 32],
    pub volume_identifier: [u8; 32],
    _pad2: [u8; 8],
    pub volume_space_size: BothEndian32,
    _pad3: [u8; 32],
    pub volume_set_size: BothEndian16,
    pub volume_sequence_number: BothEndian16,
    pub logical_block_size: BothEndian16,

    pub path_table_size: BothEndian32,
    pub path_table_loc_le: u32,
    pub optional_path_table_loc_le: u32,
    pub path_table_loc_be: u32,
    pub optional_path_table_loc_be: u32,

    _root_directory_entry: [u8; 34],

    pub volume_set_identifier: [u8; 128],
    pub publisher_identifier: [u8; 128],
    pub data_preparer_identifier: [u8; 128],
    pub application_identifier: [u8; 128],
    pub copyright_file_identifier: [u8; 38],
    pub abstract_file_identifier: [u8; 36],
    pub bibliographic_file_identifier: [u8; 37],

    // XXX create a struct for times
    pub creation_time: DateTimeAscii,
    pub modification_time: DateTimeAscii,
    pub expiration_time: DateTimeAscii,
    pub effective_time: DateTimeAscii,

    pub file_structure_version: u8,
    _pad5: [u8; 1166]
}

impl PrimaryVolumeDescriptor {
    pub fn root_directory_entry(&self) -> &DirectoryEntryHeader {
        // This deals with alignment, since PrimaryVolumeDescriptor
        // has no padding around the directory entry field, but it is
        // aligned correctly. This allows DirectoryEntryHeader to not
        // be repr(packed)

        // TODO: use safer and cleaner solution if possible
        let root_ptr = &self._root_directory_entry as *const u8;
        unsafe { &*(root_ptr.offset(-2) as *const DirectoryEntryHeader) }
    }
}

assert_eq_size!(vol_desc_size_eq; VolumeDescriptor, [u8; 2048]);
assert_eq_size!(prim_vol_desc_size_eq; PrimaryVolumeDescriptor, [u8; 2048]);
