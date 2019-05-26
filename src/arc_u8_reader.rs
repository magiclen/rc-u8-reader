use std::sync::Arc;
use std::io::{self, ErrorKind, Read, BufRead, Seek, SeekFrom};
#[cfg(feature = "nightly")]
use std::io::{IoSliceMut, Initializer};
use std::cmp;

pub struct ArcU8Reader<T: AsRef<[u8]> + ?Sized> {
    data: Arc<T>,
    pos: usize,
}

impl<T: AsRef<[u8]> + ?Sized> ArcU8Reader<T> {
    #[inline]
    pub fn new(data: Arc<T>) -> ArcU8Reader<T> {
        ArcU8Reader {
            data,
            pos: 0,
        }
    }
}

impl<T: AsRef<[u8]> + ?Sized> BufRead for ArcU8Reader<T> {
    #[inline]
    fn fill_buf(&mut self) -> Result<&[u8], io::Error> {
        let data = (*self.data).as_ref();

        let amt = cmp::min(self.pos, data.len());

        Ok(&data[amt..])
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.pos += amt;
    }
}

impl<T: AsRef<[u8]> + ?Sized> Read for ArcU8Reader<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = Read::read(&mut self.fill_buf()?, buf)?;

        self.pos += n;

        Ok(n)
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

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let n = buf.len();

        Read::read_exact(&mut self.fill_buf()?, buf)?;

        self.pos += n;

        Ok(())
    }
}

impl<T: AsRef<[u8]> + ?Sized> Seek for ArcU8Reader<T> {
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
            SeekFrom::End(n) => ((*self.data).as_ref().len(), n),
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
        Ok((*self.data).as_ref().len() as u64)
    }

    #[cfg(feature = "nightly")]
    #[inline]
    fn stream_position(&mut self) -> Result<u64, io::Error> {
        Ok(self.pos as u64)
    }
}