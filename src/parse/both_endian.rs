// SPDX-License-Identifier: (MIT OR Apache-2.0)

use nom::bytes::complete::take;
use nom::number::complete::*;
use nom::sequence::terminated;

// ISO 9660 uses a representation for integers with both little
// and big endian representations of the same number.

// This only reads the little endian version.
// The Linux kernel does the same, with a comment about some programs
// generating invalid ISO with incorrect big endian values.

named!(pub both_endian16<u16>, call!(terminated(le_u16, take(2usize))));
named!(pub both_endian32<u32>, call!(terminated(le_u32, take(4usize))));
