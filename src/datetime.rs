use time::Tm;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DateTimeAscii {
    // Other than gmt_offset, fields are ascii decimal
    pub year: [u8; 4],
    pub month: [u8; 2],
    pub day: [u8; 2],
    pub hour: [u8; 2],
    pub minute: [u8; 2],
    pub second: [u8; 2],
    pub centisecond: [u8; 2],
    pub gmt_offset: u8
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DateTime {
    pub year: u8, // years since 1900
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub gmt_offset: u8
}

impl DateTime {
    pub fn to_tm(&self) -> Tm {
        Tm {
            tm_year: self.year as i32,
            tm_mon: self.month as i32,
            tm_hour: self.hour as i32,
            tm_min: self.minute as i32,
            tm_sec: self.second as i32,
            tm_mday: self.day as i32,
            tm_wday: -1, // XXX
            tm_yday: -1, // XXX
            tm_nsec: 0,
            tm_isdst: -1,
            tm_utcoff: self.gmt_offset as i32,
        }
    }
}

assert_eq_size!(datetime_ascii_size_eq; DateTimeAscii, [u8; 17]);
assert_eq_size!(datetime_size_eq; DateTime, [u8; 7]);
