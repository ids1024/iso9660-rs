use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use ::{ISOError, Result};

/// Read the block at a given LBA (logical block address)
pub(crate) fn read_block(file: &Rc<RefCell<File>>, lba: u64) -> Result<[u8; 2048]> {
    // Using uninitialized is safe because the buffer is not read unless it is
    // entirely filled. This may not work correctly with arbitrary Read types,
    // but an API to address that is being developed:
    // https://github.com/rust-lang/rust/issues/42788

    let mut block: [u8; 2048] = unsafe { mem::uninitialized() };
    let file = (*file).borrow_mut();
    let count;

    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        count = file.read_at(&mut block, lba * 2048)?;
    }
    #[cfg(not(unix))]
    {
        use std::io::{SeekFrom, Read, Seek};
        file.seek(SeekFrom::Start(lba * 2048))?;
        count = file.read(&mut block)?;
    }

    if count != 2048 {
        Err(ISOError::BlockReadSize(count))
    } else {
        Ok(block)
    }
}
