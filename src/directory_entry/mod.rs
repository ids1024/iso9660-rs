pub use self::isodirectory::ISODirectory;
pub use self::isofile::ISOFile;

use ::parse::DirectoryEntryHeader;
use ::ISO9660Reader;

mod isodirectory;
mod isofile;

#[derive(Clone, Debug)]
pub enum DirectoryEntry<T: ISO9660Reader> {
    Directory(ISODirectory<T>),
    File(ISOFile<T>)
}

impl<T: ISO9660Reader> DirectoryEntry<T> {
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
