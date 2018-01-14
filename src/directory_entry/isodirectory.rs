use std::{cmp, mem, ptr, str};
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;

use ::{DirectoryEntry, ISOFile, read_block, Result, ISOError};
use super::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISODirectory {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    file: Rc<RefCell<File>>
}

impl ISODirectory {
    pub(crate) fn new(header: DirectoryEntryHeader, identifier: String, file: Rc<RefCell<File>>) -> ISODirectory {
        ISODirectory {
            header,
            identifier,
            file
        }
    }

    // TODO: Iterator? Perhaps using generator?
    pub fn contents(&self) -> Result<Vec<DirectoryEntry>> {
        let mut entries = Vec::new();

        let loc = *self.header.extent_loc;
        let len = *self.header.extent_length;

        for block_num in 0..(len / 2048) {
            let block_len = cmp::min(len - 2048 * block_num, 2048);
            let block = read_block(&self.file, loc as u64 + block_num as u64)?;

            let mut block_pos: u32 = 0;
            while block_pos < block_len {
                let mut header: DirectoryEntryHeader = unsafe { mem::uninitialized() };
                let entry = &block[block_pos as usize..];
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
                    return Err(ISOError::InvalidFs("length < 34"));
                }

                if header.length as u32 > 2048 - block_pos {
                    return Err(ISOError::InvalidFs("length > left on block"));
                }

                if header.length % 2 != 0 {
                    return Err(ISOError::InvalidFs("length % 2 != 0"));
                }

                if header.file_identifier_len > header.length {
                    return Err(ISOError::InvalidFs("identifer_len > length"));
                }

                // 33 is the size of the header without padding
                let end = header.file_identifier_len as usize + 33;
                let file_identifier = str::from_utf8(&entry[33..end])?;


                // After the file identifier, ISO 9660 allows addition space for
                // system use. Ignore that for now.

                block_pos += header.length as u32;

                let entry = if header.is_directory() {
                    DirectoryEntry::Directory(ISODirectory::new(
                        header,
                        file_identifier.to_string(),
                        self.file.clone()
                    ))
                } else {
                    DirectoryEntry::File(ISOFile::new(
                        header,
                        file_identifier,
                        self.file.clone()
                    )?)
                };

                entries.push(entry);
            }
        }

        Ok(entries)
    }
}
