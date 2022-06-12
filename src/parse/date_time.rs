// SPDX-License-Identifier: (MIT OR Apache-2.0)

use nom::bytes::complete::take;
use nom::number::complete::le_u8;
use std::convert::TryFrom;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

named!(pub date_time<OffsetDateTime>, do_parse!(
    year:       le_u8 >> // years since 1900
    month:      le_u8 >>
    day:        le_u8 >>
    hour:       le_u8 >>
    minute:     le_u8 >>
    second:     le_u8 >>
    gmt_offset: le_u8 >>
    ({
        // Create Date and Time from parsed values. Since those values can be 0,
        // creating Date and Time struct can fail, in this case assume default
        // values.
        let date = Date::from_calendar_date(
            1900 + year as i32,
            time::Month::try_from(month).unwrap_or(time::Month::January),
            day)
            .unwrap_or_else(|_| Date::from_calendar_date(0, time::Month::January, 1).unwrap());

        let time = Time::from_hms(hour, minute, second)
            .unwrap_or_else(|_| Time::from_hms(0, 0, 0).unwrap());

        // gmt_offset represents 15 minutes intervals from GMT.
        let offset = UtcOffset::from_whole_seconds((gmt_offset as i32) * 15 * 60)
            .unwrap_or(UtcOffset::UTC);

        PrimitiveDateTime::new(date, time).assume_offset(offset)
    })
));

named_args!(ascii_i32(n: usize)<i32>, flat_map!(take(n), parse_to!(i32)));
named!(pub date_time_ascii<OffsetDateTime>, do_parse!(
    tm_year:     call!(ascii_i32, 4) >>
    tm_mon:      call!(ascii_i32, 2) >>
    tm_mday:     call!(ascii_i32, 2) >>
    tm_hour:     call!(ascii_i32, 2) >>
    tm_min:      call!(ascii_i32, 2) >>
    tm_sec:      call!(ascii_i32, 2) >>
    centisecond: call!(ascii_i32, 2) >>
    gmt_offset:  le_u8    >>
    ({
        let date = Date::from_calendar_date(
            1900 + tm_year as i32,
            time::Month::try_from(tm_mon as u8).unwrap_or(time::Month::January),
            tm_mday as u8)
            .unwrap_or_else(|_| Date::from_calendar_date(0, time::Month::January, 1).unwrap());

        let time = Time::from_hms_milli(tm_hour as u8, tm_min as u8, tm_sec as u8, centisecond as u16 * 10)
            .unwrap_or_else(|_| Time::from_hms(0, 0, 0).unwrap());

        let offset = UtcOffset::from_whole_seconds((gmt_offset as i32) * 15 * 60)
            .unwrap_or(UtcOffset::UTC);

        PrimitiveDateTime::new(date, time).assume_offset(offset)
    })
));
