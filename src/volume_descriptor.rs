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

#[repr(C)]
pub struct PrimaryVolumeDescriptor {
    _pad1: u8,
    pub system_identifier: [u8; 32],
    pub volume_identifier: [u8; 32],
    _pad2: [u8; 8],
    pub volume_space_size: BothEndian32,
    _pad3: [u8; 32],
    pub volume_set_size: BothEndian16,
    pub volume_sequence_number: BothEndian16,
}

assert_eq_size!(vol_desc_size_eq; VolumeDescriptor, [u8; 2048]);
assert_eq_size!(prim_vol_desc_size_eq; PrimaryVolumeDescriptor, [u8; 2041]);
