use std::str::FromStr;
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;

use super::DirectoryEntryHeader;
use ::{read_block, Result, ISOError};

#[derive(Clone, Debug)]
pub struct ISOFile {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    // File version; ranges from 1 to 32767
    pub version: u16,
    file: Rc<RefCell<File>>
}

impl ISOFile {
    pub(crate) fn new(header: DirectoryEntryHeader, identifier: &str, file: Rc<RefCell<File>>) -> Result<ISOFile> {
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
        let loc = *self.header.extent_loc;
        let len = *self.header.extent_length;
        let mut buf = Vec::new();

        let mut block_num = 0;
        while block_num * 2048 < len {
            let block = read_block(&self.file, (loc + block_num) as u64)?;
            let size = len as usize - 2048 * (block_num as usize);
            buf.extend_from_slice(&block[0..size]);
            block_num += 1;
        }

        Ok(buf)
    }
}
