// SPDX-License-Identifier: (MIT OR Apache-2.0)

use nom::number::complete::*;

// ISO 9660 uses a representation for integers with both little
// and big endian representations of the same number.

named!(pub both_endian16<&[u8], u16>, do_parse!(
    // Only reading the little endian version.
    // The Linux kernel does the same, with a comment about some programs
    // generating invalid ISO with incorrect big endian values.
    val: le_u16 >>
         take!(2) >>
    (val)
));

named!(pub both_endian32<&[u8], u32>, do_parse!(
    val: le_u32 >>
         take!(4) >>
    (val)
));
