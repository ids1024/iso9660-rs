use ::volume_descriptor::DirectoryEntryHeader;

#[derive(Clone, Debug)]
pub struct ISOFile {
    pub(crate) header: DirectoryEntryHeader,
    pub(crate) identifier: String
}
