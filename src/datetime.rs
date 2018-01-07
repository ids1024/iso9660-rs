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

assert_eq_size!(datetime_ascii_size_eq; DateTimeAscii, [u8; 17]);
assert_eq_size!(datetime_size_eq; DateTime, [u8; 7]);
