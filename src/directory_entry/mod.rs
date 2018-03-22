pub use self::isodirectory::ISODirectory;
pub use self::isofile::ISOFile;

use ::parse::DirectoryEntryHeader;

mod isodirectory;
mod isofile;

#[derive(Clone, Debug)]
pub enum DirectoryEntry {
    Directory(ISODirectory),
    File(ISOFile)
}

impl DirectoryEntry {
    pub(crate) fn header(&self) -> &DirectoryEntryHeader {
        match *self {
            DirectoryEntry::Directory(ref dir) => &dir.header,
            DirectoryEntry::File(ref file) => &file.header,
        }

    }

    pub fn identifier(&self) -> &str {
        match *self {
            DirectoryEntry::Directory(ref dir) => &dir.identifier,
            DirectoryEntry::File(ref file) => &file.identifier,
        }
    }
}
