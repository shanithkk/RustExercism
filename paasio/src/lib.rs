use std::io::{Read, Result, Write};

pub struct IoState<T> {
    wrapped: T,
    bytes_through: usize,
    writes: usize,
    reads: usize,
}

pub type ReadStats<T> = IoState<T>;
pub type WriteStats<T> = IoState<T>;

impl<T> IoState<T> {
  
    pub fn new(wrapped: T) -> ReadStats<T> {
        IoState {
            wrapped,
            bytes_through: 0,
            writes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }
    pub fn writes(&self) -> usize {
        self.writes
    }
    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}