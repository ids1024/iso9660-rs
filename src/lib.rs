#![feature(untagged_unions)]

#[macro_use]
extern crate static_assertions;

use std::io::{SeekFrom, Read, Seek};
use std::fs::File;
use std::path::Path;
use std::mem;
use std::result;

use volume_descriptor::VolumeDescriptor;

pub use directory_entry::{DirectoryEntry, ISODirectory, ISOFile};
pub(crate) use fileref::FileRef;
pub use error::ISOError;

pub(crate) type Result<T> = result::Result<T, ISOError>;

mod both_endian;
mod volume_descriptor;
mod directory_entry;
mod datetime;
mod fileref;
mod error;

pub struct ISO9660 {
    _file: FileRef,
    pub root: ISODirectory
}

impl ISO9660 {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ISO9660> {
        let mut file = File::open(&path)?;
        // Using uninitialized is safe for reasons addressed in read_block.rs
        // Using VolumeDescriptor with potentially arbirary data is safe
        let mut desc: VolumeDescriptor = unsafe { mem::uninitialized() };
        let mut root = None;

        // Skip the "system area"
        file.seek(SeekFrom::Start(16 * 2048))?;

        // Read volume descriptors
        loop {
            {
                let buf = unsafe {
                    &mut *(&mut desc as *mut _ as *mut [u8; 2048])
                };

                let count = file.read(buf)?;

                if count != 2048 {
                    return Err(ISOError::ReadSize(2048, count));
                }
            }

            let header = unsafe { &desc.header };

            if (&header.identifier, header.version) != (b"CD001", 1) {
                return Err(ISOError::InvalidFs("'CD001' identifier missing"));
            }

            match header.type_code {
                // Boot record
                0 => {}
                // Primary volume descriptor
                1 => {
                    let primary = unsafe { &desc.primary };

                    if *primary.logical_block_size != 2048 {
                        // This is almost always the case, but technically
                        // not guaranteed by the standard.
                        // TODO: Implement this
                        return Err(ISOError::InvalidFs("Block size not 2048"));
                    }

                    root = Some(primary.root_directory_entry().clone());
                },
                // Supplementary volume descriptor
                2 => {}
                // Volume partition descriptor
                3 => {}
                // Volume descriptor set terminator
                255 => break,
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
        for segment in path.to_uppercase().split('/') {
            let parent = match entry {
                DirectoryEntry::Directory(dir) => dir,
                _ => return Ok(None)
            };

            entry = match parent.find(segment).unwrap() {
                Some(entry) => entry,
                None => return Ok(None)
            };

        }

        Ok(Some(entry))
    }
}
