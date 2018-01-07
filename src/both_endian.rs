// ISO 9660 uses a representation for integers with both little
// and big endian representations of the same number.

use std::ops::Deref;
use std::fmt::{Debug, Formatter, Result};

#[repr(C)]
#[derive(Clone)]
pub struct BothEndian16 {
    le: u16,
    be: u16
}

impl Deref for BothEndian16 {
    type Target = u16;

    fn deref(&self) -> &u16 {
        if cfg!(target_endian = "big") {
            &self.be
        } else {
            &self.le
        }
    }
}

impl Debug for BothEndian16 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if u16::from_le(self.le) == u16::from_be(self.be) {
            f.debug_tuple("BothEndian16")
                .field(&**self)
                .finish()
        } else {
            f.debug_struct("BothEndian16")
                .field("le", &self.le)
                .field("be", &self.be)
                .finish()
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct BothEndian32 {
    le: u32,
    be: u32
}

impl Deref for BothEndian32 {
    type Target = u32;

    fn deref(&self) -> &u32 {
        if cfg!(target_endian = "big") {
            &self.be
        } else {
            &self.le
        }
    }
}

impl Debug for BothEndian32 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if u32::from_le(self.le) == u32::from_be(self.be) {
            f.debug_tuple("BothEndian32")
                .field(&**self)
                .finish()
        } else {
            f.debug_struct("BothEndian32")
                .field("le", &self.le)
                .field("be", &self.be)
                .finish()
        }
    }
}

assert_eq_size!(bothend16_size_eq; BothEndian16, [u8; 4]);
assert_eq_size!(bothend32_size_eq; BothEndian32, [u8; 8]);
