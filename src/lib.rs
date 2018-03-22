#![feature(read_initializer)]

extern crate time;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;

use std::io::{SeekFrom, Read, Seek};
use std::fs::File;
use std::path::Path;
use std::mem;
use std::result;

pub use directory_entry::{DirectoryEntry, ISODirectory, ISOFile};
pub(crate) use fileref::FileRef;
pub use error::ISOError;
use parse::VolumeDescriptor;

pub type Result<T> = result::Result<T, ISOError>;

mod directory_entry;
mod fileref;
mod error;
mod parse;

pub struct ISO9660 {
    _file: FileRef,
    pub root: ISODirectory
}

impl ISO9660 {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ISO9660> {
        let mut file = File::open(&path)?;
        let mut buf: [u8; 2048] = unsafe { mem::uninitialized() };
        let mut root = None;

        // Skip the "system area"
        file.seek(SeekFrom::Start(16 * 2048))?;

        // Read volume descriptors
        loop {
            let count = file.read(&mut buf)?;

            if count != 2048 {
                return Err(ISOError::ReadSize(2048, count));
            }

            match VolumeDescriptor::parse(&buf)? {
                // Primary volume descriptor
                Some(VolumeDescriptor::Primary {
                    logical_block_size,
                    root_directory_entry,
                    ..
                }) => {
                    if logical_block_size != 2048 {
                        // This is almost always the case, but technically
                        // not guaranteed by the standard.
                        // TODO: Implement this
                        return Err(ISOError::InvalidFs("Block size not 2048"));
                    }

                    root = Some(root_directory_entry);

                },
                Some(VolumeDescriptor::VolumeDescriptorSetTerminator) => break,
                _ => {}
            }
        }

        let file = FileRef::new(file);
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
                root,
                "\0".to_string(), // XXX actually read from disk
                file2
                )
        })
    }

    pub fn open(&self, path: &str) -> Result<Option<DirectoryEntry>> {
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
