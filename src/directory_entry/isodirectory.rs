use std::io::{Result, Error, ErrorKind};
use std::{cmp, mem, ptr, str};
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use ::{DirectoryEntry, ISOFile, Block};
use super::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISODirectory {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    pub(crate) file: Rc<RefCell<File>>
}

impl ISODirectory {
    // TODO: Iterator? Perhaps using generator?
    pub fn contents(&self) -> Result<Vec<DirectoryEntry>> {
        let mut entries = Vec::new();

        let loc = *self.header.extent_loc;
        let len = *self.header.extent_length;

        let blocks = len / 2048;
        let mut block_num: u32 = 0;
        while block_num < blocks {
            let block_len = cmp::min(len - 2048 * block_num, 2048);
            let block = Block::read(&self.file, loc as u64 + block_num as u64)?;

            let mut block_pos: u32 = 0;
            while block_pos < block_len {
                let mut header: DirectoryEntryHeader = unsafe { mem::uninitialized() };
                let entry = unsafe { &block.bytes[block_pos as usize..] };
                unsafe {
                    // Accounts for padding, which is needed for alignment
                    // TODO: Better solution
                    ptr::copy_nonoverlapping(entry.as_ptr(),
                                             (&mut header as *mut _ as *mut u8).offset(2),
                                             33);
                }

                if header.length == 0 {
                    // XXX ?
                    break;
                }

                if header.length < 34 {
                    // XXX Change error type
                    return Err(Error::new(ErrorKind::Other, "length < 34"));
                }

                if header.length as u32 > 2048 - block_pos {
                    // XXX Change error type
                    return Err(Error::new(ErrorKind::Other, "length > left on block"));
                }

                if header.length % 2 != 0 {
                    // XXX Change error type
                    return Err(Error::new(ErrorKind::Other, "length % 2 != 0"));
                }

                if header.file_identifier_len > header.length {
                    // XXX Change error type
                    return Err(Error::new(ErrorKind::Other, "identifer_len > len"));
                }

                // 33 is the size of the header without padding
                let end = header.file_identifier_len as usize + 33;
                // XXX unwrap
                let file_identifier = str::from_utf8(&entry[33..end]).unwrap();


                // After the file identifier, ISO 9660 allows addition space for
                // system use. Ignore that for now.

                block_pos += header.length as u32;

                let entry = if header.is_directory() {
                    DirectoryEntry::Directory(ISODirectory {
                        header,
                        identifier: file_identifier.to_string(),
                        file: self.file.clone()
                    })
                } else {
                    let mut name = file_identifier;
                    let mut version = None;
                    if let Some(idx) = file_identifier.rfind(";") {
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

                    DirectoryEntry::File(ISOFile {
                        header,
                        identifier: name.to_string(),
                        version
                    })
                };

                entries.push(entry);
            }

            block_num += 1;
        }

        Ok(entries)
    }
}
