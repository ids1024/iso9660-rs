#![feature(untagged_unions)]

#[macro_use]
extern crate static_assertions;

use std::io::{SeekFrom, Read, Seek, Error, ErrorKind};
use std::fs::File;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use std::result;

use volume_descriptor::VolumeDescriptor;

pub use directory_entry::{DirectoryEntry, ISODirectory, ISOFile};
pub(crate) use read_block::read_block;
pub use error::ISOError;

pub(crate) type Result<T> = result::Result<T, ISOError>;

mod both_endian;
mod volume_descriptor;
mod directory_entry;
mod datetime;
mod read_block;
mod error;

pub struct ISO9660 {
    // TODO: Figure out if sane API possible without Rc/RefCell
    _file: Rc<RefCell<File>>,
    pub root: ISODirectory
}

impl ISO9660 {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ISO9660> {
        let mut file = File::open(&path)?;
        let mut desc: VolumeDescriptor = unsafe { mem::uninitialized() };
        let mut root = None;

        // Skip the "system area"
        file.seek(SeekFrom::Start(16 * 2048))?;

        // Read volume descriptors
        loop {
            file.read(unsafe { &mut *(&mut desc as *mut _ as *mut [u8; 2048]) })?;
            let header = unsafe { &desc.header };

            if (&header.identifier, header.version) != (b"CD001", 1) {
                // XXX Change error type
                return Err(Error::new(ErrorKind::Other, "Not ISO9660").into())
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
                        return Err(Error::new(ErrorKind::Other, "Block size not 2048").into())
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

        let file = Rc::new(RefCell::new(file));
        let file2 = file.clone();

        Ok(ISO9660 {
            _file: file,
            root: ISODirectory::new(
                root.unwrap(),
                "\0".to_string(), // XXX actually read from disk
                file2
                )
        })
    }

}
