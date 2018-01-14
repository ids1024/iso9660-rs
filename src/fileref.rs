use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Result;

// TODO: Figure out if sane API possible without Rc/RefCell
#[derive(Clone, Debug)]
pub(crate) struct FileRef(Rc<RefCell<File>>);

impl FileRef {
    pub fn new(file: File) -> FileRef {
        FileRef(Rc::new(RefCell::new(file)))
    }

    /// Read the block(s) at a given LBA (logical block address)
    pub fn read_at(&self, buf: &mut [u8], lba: u64) -> Result<usize> {
        #[allow(unused_mut)]
        let mut file = (*self.0).borrow_mut();

        #[cfg(unix)]
        {
            use std::os::unix::fs::FileExt;
            Ok(file.read_at(buf, lba * 2048)?)
        }
        #[cfg(not(unix))]
        {
            use std::io::{SeekFrom, Read, Seek};
            file.seek(SeekFrom::Start(lba * 2048))?;
            Ok(file.read(buf)?)
        }
    }
}
