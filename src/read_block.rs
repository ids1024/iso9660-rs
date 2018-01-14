use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use std::io::Result;

// NOTE: Technically the primary volume descriptor could specify a block size
// ofther than 2048. That is not common in practice, but should probably be
// supported.

/// Read the block at a given LBA (logical block address)
pub(crate) fn read_block(file: &Rc<RefCell<File>>, lba: u64) -> Result<[u8; 2048]> {
    let mut block: [u8; 2048] = unsafe { mem::uninitialized() };
    let file = (*file).borrow_mut();

    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.read_at(&mut block, lba * 2048)?;
    }
    #[cfg(not(unix))]
    {
        use std::io::{SeekFrom, Read, Seek};
        file.seek(SeekFrom::Start(lba * 2048))?;
        file.read(&mut block)?;
    }
    Ok(block)
}
