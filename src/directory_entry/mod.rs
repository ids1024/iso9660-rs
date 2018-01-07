pub use self::isodirectory::ISODirectory;
pub use self::isofile::ISOFile;

mod isodirectory;
mod isofile;

#[derive(Clone, Debug)]
pub enum DirectoryEntry {
    Directory(ISODirectory),
    File(ISOFile)
}
