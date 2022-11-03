use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufRead, ErrorKind, Read, Seek, SeekFrom};
use std::rc::Rc;

pub struct RcU8Reader<T: AsRef<[u8]> + ?Sized> {
    data: Rc<T>,
    pos: usize,
}

impl<T: AsRef<[u8]> + ?Sized> Debug for RcU8Reader<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        debug_helper::impl_debug_for_struct!(RcU8Reader, f, self, let .data = self.data.as_ref().as_ref(), .pos);
    }
}

impl<T: AsRef<[u8]> + ?Sized> RcU8Reader<T> {
    #[inline]
    pub fn new(data: Rc<T>) -> RcU8Reader<T> {
        RcU8Reader {
            data,
            pos: 0,
        }
    }
}

impl<T: AsRef<[u8]> + ?Sized> BufRead for RcU8Reader<T> {
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

impl<T: AsRef<[u8]> + ?Sized> Read for RcU8Reader<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = Read::read(&mut self.fill_buf()?, buf)?;

        self.pos += n;

        Ok(n)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let n = buf.len();

        Read::read_exact(&mut self.fill_buf()?, buf)?;

        self.pos += n;

        Ok(())
    }
}

impl<T: AsRef<[u8]> + ?Sized> Seek for RcU8Reader<T> {
    fn seek(&mut self, style: SeekFrom) -> Result<u64, io::Error> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                let n = if n > usize::MAX as u64 {
                    usize::MAX
                } else {
                    n as usize
                };

                self.pos = n;

                return Ok(n as u64);
            }
            SeekFrom::End(n) => ((*self.data).as_ref().len(), n),
            SeekFrom::Current(n) => (self.pos, n),
        };

        let offset = if offset > isize::MAX as i64 {
            isize::MAX
        } else if offset < isize::MIN as i64 {
            isize::MIN
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
            None => {
                Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "invalid seek to a negative or overflowing position",
                ))
            }
        }
    }
}
