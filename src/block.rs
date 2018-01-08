use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use std::io::Result;

use volume_descriptor::{VolumeDescriptor};

#[repr(C)]
pub(crate) union Block {
    // CDROMs contain 2048 byte blocks
    pub(crate) bytes: [u8; 2048],
    pub(crate) volume_descriptor: VolumeDescriptor
}

impl Block {
    /// Read the block at a given LBA (logical block address)
    pub(crate) fn read(file: &Rc<RefCell<File>>, lba: u64) -> Result<Block> {
        let mut block: Block = unsafe { mem::uninitialized() };
        let file = (*file).borrow_mut();

        #[cfg(unix)]
        {
            use std::os::unix::fs::FileExt;
            file.read_at(unsafe { &mut block.bytes }, lba * 2048)?;
        }
        #[cfg(not(unix))]
        {
            use std::io::{SeekFrom, Read, Seek};
            file.seek(SeekFrom::Start(lba * 2048))?;
            file.read(unsafe { &mut block.bytes })?;
        }
        Ok(block)
    }
}
