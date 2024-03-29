use std::ops::{Range, RangeBounds};

use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct PacketBuf(pub Bytes);
#[derive(Debug, Clone)]
pub struct PacketBufMut(pub BytesMut);

impl PacketBuf {
    pub fn copy_from_slice(slice: &[u8]) -> Self {
        Self(Bytes::copy_from_slice(slice))
    }

    pub fn slice(&self, range: Range<usize>) -> &[u8] {
        &self.0[range]
    }

    pub fn slice_utf8(&self, range: Range<usize>) -> anyhow::Result<&str> {
        let bytes = &self.0[range];
        let s = std::str::from_utf8(bytes)?;
        Ok(s)
    }
}

impl PacketBufMut {
    pub fn new() -> Self {
        Self(BytesMut::new())
    }

    pub fn with_capacity(size: usize) -> Self {
        let b = BytesMut::with_capacity(size);
        Self(b)
    }

    pub fn freeze(self) -> PacketBuf {
        PacketBuf(self.0.freeze())
    }

    pub fn slice(&self, range: Range<usize>) -> &[u8] {
        &self.0[range]
    }

    pub fn slice_mut(&mut self, range: Range<usize>) -> &[u8] {
        &mut self.0[range]
    }
}

impl From<Bytes> for PacketBuf {
    fn from(value: Bytes) -> Self {
        Self(value)
    }
}

impl From<BytesMut> for PacketBuf {
    fn from(value: BytesMut) -> Self {
        Self(Bytes::from(value))
    }
}
