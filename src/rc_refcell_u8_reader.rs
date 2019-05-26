use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};
#[cfg(feature = "nightly")]
use std::io::{IoSliceMut, Initializer};

pub struct RcRefCellU8Reader<T: AsRef<[u8]> + ?Sized> {
    data: Rc<RefCell<T>>,
    pos: usize,
}

impl<T: AsRef<[u8]> + ?Sized> RcRefCellU8Reader<T> {
    #[inline]
    pub fn new(data: Rc<RefCell<T>>) -> RcRefCellU8Reader<T> {
        RcRefCellU8Reader {
            data,
            pos: 0,
        }
    }
}

impl<T: AsRef<[u8]> + ?Sized> Read for RcRefCellU8Reader<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let data: Ref<T> = (*self.data).borrow();
        let data: &[u8] = &data.as_ref()[self.pos..];

        let data_len = data.len();
        let buf_len = buf.len();

        let len = if data_len > buf_len {
            let data = &data[..buf_len];

            buf.copy_from_slice(data);

            buf_len
        } else {
            buf[..data_len].copy_from_slice(data);

            data_len
        };

        self.pos += len;

        Ok(len)
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        let mut nread = 0;
        for buf in bufs {
            let n = self.read(buf)?;
            nread += n;
            if n < buf.len() {
                break;
            }
        }
        Ok(nread)
    }

    #[cfg(feature = "nightly")]
    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::nop()
    }
}

impl<T: AsRef<[u8]> + ?Sized> Seek for RcRefCellU8Reader<T> {
    fn seek(&mut self, style: SeekFrom) -> Result<u64, io::Error> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                let n = if n > usize::max_value() as u64 {
                    usize::max_value()
                } else {
                    n as usize
                };

                self.pos = n;

                return Ok(n as u64);
            }
            SeekFrom::End(n) => (
                {
                    let data: Ref<T> = (*self.data).borrow();
                    let data: &[u8] = &data.as_ref()[self.pos..];

                    data.len()
                },
                n
            ),
            SeekFrom::Current(n) => (self.pos, n),
        };

        let offset = if offset > isize::max_value() as i64 {
            isize::max_value()
        } else if offset < isize::min_value() as i64 {
            isize::min_value()
        } else {
            offset as isize
        };

        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as usize)
        } else {
            base_pos.checked_sub((offset.wrapping_neg()) as usize)
        };

        match new_pos {
            Some(n) => {
                self.pos = n;

                Ok(self.pos as u64)
            }
            None => Err(io::Error::new(ErrorKind::InvalidInput, "invalid seek to a negative or overflowing position"))
        }
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn stream_len(&mut self) -> Result<u64, io::Error> {
        let data_len = {
            let data: Ref<T> = (*self.data).borrow();
            let data: &[u8] = &data.as_ref()[self.pos..];

            data.len()
        };

        Ok(data_len as u64)
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn stream_position(&mut self) -> Result<u64, io::Error> {
        Ok(self.pos as u64)
    }
}