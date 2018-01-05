use both_endian::{BothEndian16, BothEndian32};


#[repr(C, align(32))]
pub struct VolumeDescriptor {
    pub type_code: u8,
    pub identifier: [u8; 5],
    pub version: u8,
    pub data: VolumeDescriptorData
}

#[repr(C)]
pub union VolumeDescriptorData {
    primary: PrimaryVolumeDescriptor
}

#[repr(C, packed)]
pub struct PrimaryVolumeDescriptor {
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

    pub root_directory_entry: [u8; 34], // XXX change type to DirectoryEntry

    pub volume_set_identifier: [u8; 128],
    pub publisher_identifier: [u8; 128],
    pub data_preparer_identifier: [u8; 128],
    pub application_identifier: [u8; 128],
    pub copyright_file_identifier: [u8; 38],
    pub abstract_file_identifier: [u8; 36],
    pub bibliographic_file_identifier: [u8; 37],

    // XXX create a struct for times
    pub creation_time: DateTime,
    pub modification_time: DateTime,
    pub expiration_time: DateTime,
    pub effective_time: DateTime,

    pub file_structure_version: u8,
    _pad4: [u8; 1166]
}

#[repr(C, packed)]
pub struct DateTime {
    // Other than gmt_offset, fields are ascii decimal
    pub year: [u8; 4],
    pub month: [u8; 2],
    pub day: [u8; 2],
    pub hour: [u8; 2],
    pub minute: [u8; 2],
    pub second: [u8; 2],
    pub centisecond: [u8; 2],
    pub gmt_offset: u8
}

assert_eq_size!(vol_desc_size_eq; VolumeDescriptor, [u8; 2048]);
assert_eq_size!(prim_vol_desc_size_eq; PrimaryVolumeDescriptor, [u8; 2041]);
