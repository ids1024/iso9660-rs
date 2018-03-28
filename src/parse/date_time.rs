use std::str::{self, FromStr};

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
    year:        take!(4) >>
    month:       take!(2) >>
    day:         take!(2) >>
    hour:        take!(2) >>
    minute:      take!(2) >>
    second:      take!(2) >>
    centisecond: take!(2) >>
    gmt_offset:  le_u8    >>
    (Tm {
        // XXX unwrap
        tm_year: i32::from_str(str::from_utf8(year).unwrap()).unwrap(),
        tm_mon: i32::from_str(str::from_utf8(month).unwrap()).unwrap(),
        tm_hour: i32::from_str(str::from_utf8(hour).unwrap()).unwrap(),
        tm_min: i32::from_str(str::from_utf8(minute).unwrap()).unwrap(),
        tm_sec: i32::from_str(str::from_utf8(second).unwrap()).unwrap(),
        tm_mday: i32::from_str(str::from_utf8(day).unwrap()).unwrap(),
        tm_wday: -1, // XXX
        tm_yday: -1, // XXX
        tm_nsec: i32::from_str(str::from_utf8(centisecond).unwrap()).unwrap() * 10_000_000,
        tm_isdst: -1,
        tm_utcoff: gmt_offset as i32,
    })
));
