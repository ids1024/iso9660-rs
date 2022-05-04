// SPDX-License-Identifier: (MIT OR Apache-2.0)

use nom::bytes::complete::take;
use nom::number::complete::le_u8;
use time::Tm;

named!(pub date_time<Tm>, do_parse!(
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

named_args!(ascii_i32(n: usize)<i32>, flat_map!(take(n), parse_to!(i32)));
named!(pub date_time_ascii<Tm>, do_parse!(
    tm_year:     call!(ascii_i32, 4) >>
    tm_mon:      call!(ascii_i32, 2) >>
    tm_mday:     call!(ascii_i32, 2) >>
    tm_hour:     call!(ascii_i32, 2) >>
    tm_min:      call!(ascii_i32, 2) >>
    tm_sec:      call!(ascii_i32, 2) >>
    centisecond: call!(ascii_i32, 2) >>
    gmt_offset:  le_u8    >>
    (Tm {
        tm_year,
        tm_mon,
        tm_hour,
        tm_min,
        tm_sec,
        tm_mday,
        tm_wday: -1, // XXX
        tm_yday: -1, // XXX
        tm_nsec: centisecond * 10_000_000,
        tm_isdst: -1,
        tm_utcoff: gmt_offset as i32,
    })
));
