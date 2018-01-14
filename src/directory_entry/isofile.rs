use std::str::FromStr;

use super::DirectoryEntryHeader;
use ::{FileRef, Result, ISOError};

#[derive(Clone, Debug)]
pub struct ISOFile {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    // File version; ranges from 1 to 32767
    pub version: u16,
    file: FileRef
}

impl ISOFile {
    pub(crate) fn new(header: DirectoryEntryHeader, identifier: &str, file: FileRef) -> Result<ISOFile> {
        // Files (not directories) in ISO 9660 have a version number, which is
        // provided at the end of the identifier, seperated by ';'
        let error = ISOError::InvalidFs("File indentifier missing ';'");
        let idx = identifier.rfind(";").ok_or(error)?;

        let ver_str = &identifier[idx+1..];
        let mut name = &identifier[..idx];
        let version = u16::from_str(ver_str)?;

        // Files without an extension have a '.' at the end
        name = name.trim_right_matches('.');

        Ok(ISOFile {
            header,
            identifier: name.to_string(),
            version,
            file
        })
    }

    pub fn read(&self) -> Result<Vec<u8>> {
        let loc = *self.header.extent_loc as u64;
        let len = *self.header.extent_length as usize;

        // Should use safe API if possible:
        // https://github.com/rust-lang/rust/issues/42788
        let mut buf = Vec::with_capacity(len);
        unsafe { buf.set_len(len) };

        let count = self.file.read_at(buf.as_mut_slice(), loc)?;

        if count == len {
            Ok(buf)
        } else {
            Err(ISOError::ReadSize(len, count))
        }
    }
}
