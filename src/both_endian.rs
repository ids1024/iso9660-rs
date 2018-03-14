// ISO 9660 uses a representation for integers with both little
// and big endian representations of the same number.

use std::fmt::{Debug, Formatter, Result};

use byteorder::{ByteOrder, LittleEndian};

#[repr(C)]
#[derive(Clone)]
pub struct BothEndian16 {
    le: [u8; 2],
    be: [u8; 2]
}

impl BothEndian16 {
    pub(crate) fn get(&self) -> u16 {
        // Only reading the little endian version, which is first.
        // The Linux kernel does the same, with a comment about some programs
        // generating invalid ISO with incorrect big endian values.
        LittleEndian::read_u16(&self.le)
    }
}

impl Debug for BothEndian16 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_tuple("BothEndian16")
            .field(&self.get())
            .finish()
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct BothEndian32{
    le: [u8; 4],
    be: [u8; 4]
}

impl BothEndian32 {
    pub(crate) fn get(&self) -> u32 {
        LittleEndian::read_u32(&self.le)
    }
}

impl Debug for BothEndian32 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_tuple("BothEndian32")
            .field(&self.get())
            .finish()
    }
}

assert_eq_size!(bothend16_size_eq; BothEndian16, [u8; 4]);
assert_eq_size!(bothend32_size_eq; BothEndian32, [u8; 8]);
