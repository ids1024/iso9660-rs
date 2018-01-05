use both_endian::{BothEndian16, BothEndian32};


#[repr(C)]
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

#[repr(packed)]
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
    pub creation_time: [u8; 17],
    pub modification_time: [u8; 17],
    pub expiration_time: [u8; 17],
    pub effective_time: [u8; 17],

    pub file_structure_version: u8,
    _pad4: [u8; 1166]
}

assert_eq_size!(vol_desc_size_eq; VolumeDescriptor, [u8; 2048]);
assert_eq_size!(prim_vol_desc_size_eq; PrimaryVolumeDescriptor, [u8; 2041]);
