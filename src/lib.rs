#![feature(untagged_unions)]

#[macro_use]
extern crate static_assertions;

use std::io::{Result, SeekFrom, Read, Seek, Error, ErrorKind};
use std::fs::File;
use std::path::Path;

use volume_descriptor::{VolumeDescriptor, DirectoryEntryHeader};

mod both_endian;
mod volume_descriptor;


#[repr(C)]
union Block {
    // CDROMs contain 2048 byte blocks
    bytes: [u8; 2048],
    volume_descriptor: VolumeDescriptor
}

pub struct ISO9660 {
    file: File,
    block: Block,
    root: DirectoryEntryHeader
}

impl ISO9660 {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ISO9660> {
        let mut file = File::open(&path)?;
        let mut block = Block { bytes: [0; 2048] };
        let mut root = None;

        // Skip the "system area"
        file.seek(SeekFrom::Start(16 * 2048))?;

        // Read volume descriptors
        loop {
            file.read(unsafe { &mut block.bytes })?;
            let desc = unsafe { &block.volume_descriptor };
            let header = unsafe { &desc.header };

            if (&header.identifier, header.version) != (b"CD001", 1) {
                // XXX Change error type
                return Err(Error::new(ErrorKind::Other, "Not ISO9660"))
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
                        return Err(Error::new(ErrorKind::Other, "Block size not 2048"))
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

        Ok(ISO9660 {
            file,
            block: Block { bytes: [0; 2048] },
            root: root.unwrap()
        })
    }

    /// Read the block at a given LBA (logical block address)
    fn read_block(&mut self, lba: u64) -> Result<&Block> {
        self.file.seek(SeekFrom::Start(lba * 2048))?;
        self.file.read(unsafe { &mut self.block.bytes })?;
        Ok(&self.block)
    }
}

assert_eq_size!(block_size_eq; Block, [u8; 2048]);
