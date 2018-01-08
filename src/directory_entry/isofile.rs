use std::str::FromStr;

use super::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISOFile {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    // File version; ranges from 1 to 32767
    pub version: u16
}

impl ISOFile {
    pub(crate) fn new(header: DirectoryEntryHeader, identifier: &str) -> ISOFile {
        // Files (not directories) in ISO 9660 have a version number, which is
        // provided at the end of the identifier, seperated by ';'
        // XXX unwrap
        let idx = identifier.rfind(";").unwrap();

        let ver_str = &identifier[idx+1..];
        let mut name = &identifier[..idx];
        // XXX unwrap
        let version = u16::from_str(ver_str).unwrap();

        // Files without an extension have a '.' at the end
        name = name.trim_right_matches('.');

        ISOFile {
            header,
            identifier: name.to_string(),
            version
        }
    }
}
