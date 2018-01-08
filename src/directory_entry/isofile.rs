use super::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISOFile {
    pub(crate) header: DirectoryEntryHeader,
    pub identifier: String,
    // File version; ranges from 1 to 32767
    pub version: Option<u16>
}
