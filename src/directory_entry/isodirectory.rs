use std::io::{Result, Error, ErrorKind};
use std::{cmp, mem, ptr, str};

use ::{ISO9660, DirectoryEntry, ISOFile};
use super::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISODirectory {
    pub(crate) header: DirectoryEntryHeader,
    pub(crate) identifier: String
}

impl ISODirectory {
    // TODO: Iterator? Perhaps using generator?
    pub fn contents(&self, fs: &ISO9660) -> Result<Vec<DirectoryEntry>> {
        let mut entries = Vec::new();

        let loc = *self.header.extent_loc;
        let len = *self.header.extent_length;

        let blocks = len / 2048;
        let mut block_num: u32 = 0;
        while block_num < blocks {
            let block_len = cmp::min(len - 2048 * block_num, 2048);
            let block = fs.read_block(loc as u64 + block_num as u64)?;

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
                let file_identifier = str::from_utf8(&entry[33..end]).unwrap().to_string();


                // After the file identifier, ISO 9660 allows addition space for
                // system use. Ignore that for now.

                block_pos += header.length as u32;

                let entry = if header.is_directory() {
                    DirectoryEntry::Directory(ISODirectory {
                        header,
                        identifier: file_identifier
                    })
                } else {
                    DirectoryEntry::File(ISOFile {
                        header,
                        identifier: file_identifier
                    })
                };

                entries.push(entry);
            }

            block_num += 1;
        }

        Ok(entries)
    }
}
