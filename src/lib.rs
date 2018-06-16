#![feature(read_initializer)]

extern crate time;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;

use std::mem;
use std::result;

pub use directory_entry::{DirectoryEntry, ISODirectory, ISOFile};
pub(crate) use fileref::FileRef;
pub use fileref::ISO9660Reader;
pub use error::ISOError;
use parse::VolumeDescriptor;

pub type Result<T> = result::Result<T, ISOError>;

mod directory_entry;
mod fileref;
mod error;
mod parse;

pub struct ISO9660<T: ISO9660Reader> {
    _file: FileRef<T>,
    pub root: ISODirectory<T>
}

impl<T: ISO9660Reader> ISO9660<T> {
    pub fn new(mut reader: T) -> Result<ISO9660<T>> {
        let mut buf: [u8; 2048] = unsafe { mem::uninitialized() };
        let mut root = None;

        // Skip the "system area"
        let mut lba = 16;

        // Read volume descriptors
        loop {
            let count = reader.read_at(&mut buf, lba)?;

            if count != 2048 {
                return Err(ISOError::ReadSize(2048, count));
            }

            match VolumeDescriptor::parse(&buf)? {
                Some(VolumeDescriptor::Primary {
                    logical_block_size,
                    root_directory_entry,
                    root_directory_entry_identifier,
                    ..
                }) => {
                    if logical_block_size != 2048 {
                        // This is almost always the case, but technically
                        // not guaranteed by the standard.
                        // TODO: Implement this
                        return Err(ISOError::InvalidFs("Block size not 2048"));
                    }

                    root = Some((root_directory_entry, root_directory_entry_identifier));

                },
                Some(VolumeDescriptor::VolumeDescriptorSetTerminator) => break,
                _ => {}
            }

            lba += 1;
        }

        let file = FileRef::new(reader);
        let file2 = file.clone();

        let root = match root {
            Some(root) => root,
            None => {
                return Err(ISOError::InvalidFs("No primary volume descriptor"));
            }
        };

        Ok(ISO9660 {
            _file: file,
            root: ISODirectory::new(
                root.0,
                root.1,
                file2
                )
        })
    }

    pub fn open(&self, path: &str) -> Result<Option<DirectoryEntry<T>>> {
        // TODO: avoid clone()
        let mut entry = DirectoryEntry::Directory(self.root.clone());
        for segment in path.split('/').filter(|x| !x.is_empty()) {
            let parent = match entry {
                DirectoryEntry::Directory(dir) => dir,
                _ => return Ok(None)
            };

            entry = match parent.find(segment)? {
                Some(entry) => entry,
                None => return Ok(None)
            };
        }

        Ok(Some(entry))
    }

    pub fn block_size(&self) -> u16 {
        2048 // XXX
    }
}
