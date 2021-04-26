use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufRead, ErrorKind, Read, Seek, SeekFrom};
use std::sync::Arc;

#[cfg(feature = "tokio")]
use std::pin::Pin;

#[cfg(feature = "tokio")]
use std::task::{Context, Poll};

#[cfg(feature = "tokio")]
use crate::tokio::io::{AsyncRead, AsyncSeek, ReadBuf};

pub struct ArcU8Reader<T: AsRef<[u8]> + ?Sized> {
    data: Arc<T>,
    pos: usize,
}

impl<T: AsRef<[u8]> + ?Sized> Debug for ArcU8Reader<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        impl_debug_for_struct!(ArcU8Reader, f, self, let .data = self.data.as_ref().as_ref(), .pos);
    }
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
            None => {
                Err(io::Error::new(
                    ErrorKind::InvalidInput,
                    "invalid seek to a negative or overflowing position",
                ))
            }
        }
    }
}

#[cfg(feature = "tokio")]
impl<T: AsRef<[u8]> + ?Sized> AsyncRead for ArcU8Reader<T> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
        let data = (*self.data).as_ref();

        let amt = cmp::min(self.pos, data.len());

        let data = &data[amt..];

        let read_size = cmp::min(data.len(), buf.remaining());

        buf.put_slice(&data[..read_size]);

        self.pos += read_size;

        Poll::Ready(Ok(()))
    }
}

#[cfg(feature = "tokio")]
impl<T: AsRef<[u8]> + ?Sized> AsyncSeek for ArcU8Reader<T> {
    #[inline]
    fn start_seek(mut self: Pin<&mut Self>, pos: SeekFrom) -> Result<(), io::Error> {
        Seek::seek(&mut *self, pos).map(drop)
    }

    #[inline]
    fn poll_complete(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<u64>> {
        Poll::Ready(Ok(self.pos as u64))
    }
}
