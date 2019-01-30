// SPDX-License-Identifier: (MIT OR Apache-2.0)

use nom::le_u8;
use time::Tm;

named!(pub date_time<&[u8], Tm>, do_parse!(
    year:       le_u8 >> // years since 1900
    month:      le_u8 >>
    day:        le_u8 >>
    hour:       le_u8 >>
    minute:     le_u8 >>
    second:     le_u8 >>
    gmt_offset: le_u8 >>
    (Tm {
        tm_year: year as i32,
        tm_mon: month as i32,
        tm_hour: hour as i32,
        tm_min: minute as i32,
        tm_sec: second as i32,
        tm_mday: day as i32,
        tm_wday: -1, // XXX
        tm_yday: -1, // XXX
        tm_nsec: 0,
        tm_isdst: -1,
        tm_utcoff: gmt_offset as i32,
    })

));

named!(pub date_time_ascii<&[u8], Tm>, do_parse!(
    year:        flat_map!(take!(4), parse_to!(i32)) >>
    month:       flat_map!(take!(2), parse_to!(i32)) >>
    day:         flat_map!(take!(2), parse_to!(i32)) >>
    hour:        flat_map!(take!(2), parse_to!(i32)) >>
    minute:      flat_map!(take!(2), parse_to!(i32)) >>
    second:      flat_map!(take!(2), parse_to!(i32)) >>
    centisecond: flat_map!(take!(2), parse_to!(i32)) >>
    gmt_offset:  le_u8    >>
    (Tm {
        tm_year: year,
        tm_mon: month,
        tm_hour: hour,
        tm_min: minute,
        tm_sec: second,
        tm_mday: day,
        tm_wday: -1, // XXX
        tm_yday: -1, // XXX
        tm_nsec: centisecond * 10_000_000,
        tm_isdst: -1,
        tm_utcoff: gmt_offset as i32,
    })
));
