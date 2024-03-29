use std::{
    fmt::{self, Debug, Formatter},
    io::{self, ErrorKind, Read, Seek, SeekFrom},
    sync::{Arc, Mutex},
};

pub struct ArcMutexU8Reader<T: AsRef<[u8]> + ?Sized> {
    data: Arc<Mutex<T>>,
    pos:  usize,
}

impl<T: AsRef<[u8]> + ?Sized> Debug for ArcMutexU8Reader<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        debug_helper::impl_debug_for_struct!(ArcMutexU8Reader, f, self, let .data = self.data.as_ref().lock().unwrap().as_ref(), .pos);
    }
}

impl<T: AsRef<[u8]> + ?Sized> ArcMutexU8Reader<T> {
    #[inline]
    pub fn new(data: Arc<Mutex<T>>) -> ArcMutexU8Reader<T> {
        ArcMutexU8Reader {
            data,
            pos: 0,
        }
    }
}

impl<T: AsRef<[u8]> + ?Sized> Read for ArcMutexU8Reader<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let data = self.data.lock().unwrap();
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
}

impl<T: AsRef<[u8]> + ?Sized> Seek for ArcMutexU8Reader<T> {
    fn seek(&mut self, style: SeekFrom) -> Result<u64, io::Error> {
        let (base_pos, offset) = match style {
            SeekFrom::Start(n) => {
                let n = if n > usize::MAX as u64 { usize::MAX } else { n as usize };

                self.pos = n;

                return Ok(n as u64);
            },
            SeekFrom::End(n) => (
                {
                    let data = self.data.lock().unwrap();
                    let data: &[u8] = &data.as_ref()[self.pos..];

                    data.len()
                },
                n,
            ),
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
            },
            None => Err(io::Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}
