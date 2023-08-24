// SPDX-License-Identifier: (MIT OR Apache-2.0)

#![cfg_attr(feature = "nightly", feature(read_initializer, specialization))]

extern crate time;
#[macro_use]
extern crate bitflags;
extern crate nom;

use std::result;

pub use directory_entry::{
    DirectoryEntry, ISODirectory, ISODirectoryIterator, ISOFile, ISOFileReader,
};
pub use error::ISOError;
pub(crate) use fileref::FileRef;
pub use fileref::ISO9660Reader;
use parse::VolumeDescriptor;

pub type Result<T> = result::Result<T, ISOError>;

mod directory_entry;
mod error;
mod fileref;
mod parse;

pub struct ISO9660<T: ISO9660Reader> {
    _file: FileRef<T>,
    pub root: ISODirectory<T>,
    pub sup_root: Option<ISODirectory<T>>,
    primary: VolumeDescriptor,
}

macro_rules! primary_prop_str {
    ($name:ident) => {
        pub fn $name(&self) -> &str {
            if let VolumeDescriptor::Primary(table) = &self.primary {
                &table.$name
            } else {
                unreachable!()
            }
        }
    };
}

impl<T: ISO9660Reader> ISO9660<T> {
    pub fn new(mut reader: T) -> Result<ISO9660<T>> {
        let mut buf: [u8; 2048] = [0; 2048];

        let mut root = None;
        let mut primary = None;

        let mut sup_root = None;

        // Skip the "system area"
        let mut lba = 16;

        // Read volume descriptors
        loop {
            let count = reader.read_at(&mut buf, lba)?;

            if count != 2048 {
                return Err(ISOError::ReadSize(2048, count));
            }

            let descriptor = VolumeDescriptor::parse(&buf)?;
            match &descriptor {
                Some(VolumeDescriptor::Primary(table)) => {
                    if table.logical_block_size != 2048 {
                        // This is almost always the case, but technically
                        // not guaranteed by the standard.
                        // TODO: Implement this
                        return Err(ISOError::InvalidFs("Block size not 2048"));
                    }

                    root = Some((
                        table.root_directory_entry.clone(),
                        table.root_directory_entry_identifier.clone(),
                    ));
                    primary = descriptor;
                },
                Some(VolumeDescriptor::Supplementary(table)) => {
                    if table.logical_block_size != 2048 {
                        // This is almost always the case, but technically
                        // not guaranteed by the standard.
                        // TODO: Implement this
                        return Err(ISOError::InvalidFs("Block size not 2048"));
                    }

                    sup_root = Some((
                        table.root_directory_entry.clone(),
                        table.root_directory_entry_identifier.clone(),
                    ));
                }
                Some(VolumeDescriptor::VolumeDescriptorSetTerminator) => break,
                _ => {}
            }

            lba += 1;
        }

        let file = FileRef::new(reader);
        let file2 = file.clone();
        let file3 = file.clone();

        let (root, primary) = match (root, primary) {
            (Some(root), Some(primary)) => (root, primary),
            _ => {
                return Err(ISOError::InvalidFs("No primary volume descriptor"));
            }
        };

        Ok(ISO9660 {
            _file: file,
            root: ISODirectory::new(root.0, root.1, file2),
            sup_root: sup_root.map(|sup_root| ISODirectory::new(sup_root.0, sup_root.1, file3)),
            primary,
        })
    }

    pub fn open(&self, path: &str) -> Result<Option<DirectoryEntry<T>>> {
        // Try finding the path in the supplementary directory structure if it exists
        // If there's no supplementary structure or it doesn't have the file,
        // check the primary.
        Ok(
            self.sup_root
                .as_ref()
                .and_then(|sup_root| sup_root.resolve_path(path).transpose())
                .or_else(|| self.root.resolve_path(path).transpose())
                .transpose()?
        )
    }

    pub fn block_size(&self) -> u16 {
        2048 // XXX
    }

    primary_prop_str!(volume_set_identifier);
    primary_prop_str!(publisher_identifier);
    primary_prop_str!(data_preparer_identifier);
    primary_prop_str!(application_identifier);
    primary_prop_str!(copyright_file_identifier);
    primary_prop_str!(abstract_file_identifier);
    primary_prop_str!(bibliographic_file_identifier);
}
