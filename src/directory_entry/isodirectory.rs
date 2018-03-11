use std::{mem, ptr, str};

use ::{DirectoryEntry, ISOFile, FileRef, Result, ISOError};
use super::DirectoryEntryHeader;

// Like try!, but wrap in Some()
macro_rules! try_some {
    ( $res:expr ) => {
        match $res {
            Ok(val) => val,
            Err(err) => {
                return Some(Err(err.into()));
            }
        }
    };
}

#[derive(Clone, Debug)]
pub struct ISODirectory {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    file: FileRef
}

impl ISODirectory {
    pub(crate) fn new(header: DirectoryEntryHeader, identifier: String, file: FileRef) -> ISODirectory {
        ISODirectory {
            header,
            identifier,
            file
        }
    }

    // TODO: Iterator? Perhaps using generator?
    pub fn contents(&self) -> ISODirectoryIterator {
        let len = *self.header.extent_length;

        ISODirectoryIterator {
            loc: *self.header.extent_loc,
            block_count: (len + 2048 - 1) / 2048, // ceil(len / 2048)
            file: self.file.clone(),
            block: unsafe { mem::uninitialized() },
            block_num: 0,
            block_pos: 0,
            have_block: false
        }
    }
}

pub struct ISODirectoryIterator {
    loc: u32,
    block_count: u32,
    file: FileRef,
    block: [u8; 2048],
    block_num: u32,
    block_pos: u32,
    have_block: bool
}

impl Iterator for ISODirectoryIterator {
    type Item = Result<DirectoryEntry>;

    fn next(&mut self) -> Option<Result<DirectoryEntry>> {
        if self.block_num == self.block_count {
            return None;
        }

        // If we have reached the end of one block, read another
        if !self.have_block ||
           self.block_pos >= 2048 ||
           // All bytes after the last directory entry are zero.
           self.block[self.block_pos as usize] == 0 {

            if self.have_block {
                self.block_num += 1;
            }
            self.block_pos = 0;
            self.have_block = true;

            if self.block_num == self.block_count {
                return None;
            }

            let count = try_some!(self.file.read_at(
                    &mut self.block,
                    self.loc as u64 + self.block_num as u64));

            if count != 2048 {
                return Some(Err(ISOError::ReadSize(2048, count)));
            }
         }

        let entry = &self.block[self.block_pos as usize..];
        let mut header: DirectoryEntryHeader = unsafe { mem::uninitialized() };
        unsafe {
            // Accounts for padding, which is needed for alignment
            // TODO: Better solution
            ptr::copy_nonoverlapping(entry.as_ptr(),
                                     (&mut header as *mut _ as *mut u8).offset(2),
                                     33);
        }

        if header.length < 34 {
            return Some(Err(ISOError::InvalidFs("length < 34")));
        }

        if header.length as u32 > 2048 - self.block_pos {
            return Some(Err(ISOError::InvalidFs("length > left on block")));
        }

        if header.length % 2 != 0 {
            return Some(Err(ISOError::InvalidFs("length % 2 != 0")));
        }

        if header.file_identifier_len > header.length {
            return Some(Err(ISOError::InvalidFs("identifer_len > length")));
        }

        // 33 is the size of the header without padding
        let end = header.file_identifier_len as usize + 33;
        let file_identifier = try_some!(str::from_utf8(&entry[33..end]));

        // After the file identifier, ISO 9660 allows addition space for
        // system use. Ignore that for now.

        self.block_pos += header.length as u32;

        let entry = if header.is_directory() {
            DirectoryEntry::Directory(ISODirectory::new(
                header,
                file_identifier.to_string(),
                self.file.clone()
            ))
        } else {
            DirectoryEntry::File(try_some!(ISOFile::new(
                header,
                file_identifier,
                self.file.clone()
            )))
        };

        Some(Ok(entry))
    }
}
