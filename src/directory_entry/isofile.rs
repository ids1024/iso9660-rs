use std::str::FromStr;

use super::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISOFile {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    // File version; ranges from 1 to 32767
    pub version: Option<u16>
}

impl ISOFile {
    pub(crate) fn new(header: DirectoryEntryHeader, identifier: &str) -> ISOFile {
        let mut name = identifier;
        let mut version = None;

        if let Some(idx) = identifier.rfind(";") {
            // Files (not directories) in ISO 9660 can have a version
            // number, which is provided at the end of the
            // identifier, seperated by ;
            let ver_str = &name[idx+1..];
            name = &name[..idx];
            // XXX unwrap
            version = Some(u16::from_str(ver_str).unwrap())
        }

        // Files without an extension in ISO 9660 have a . at the end
        name = name.trim_right_matches('.');

        ISOFile {
            header,
            identifier: name.to_string(),
            version
        }
    }
}
