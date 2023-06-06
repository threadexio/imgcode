use std::io::{self, prelude::*};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Header {
    pub size: u64,
}

impl Header {
    pub const SIZE: usize = 8;

    /// Write the header to `writer` returning the number
    /// of bytes written.
    pub fn write_to<W>(&self, mut writer: W) -> io::Result<()>
    where
        W: Write,
    {
        let _ = writer.write(&self.size.to_be_bytes())?;
        Ok(())
    }

    /// Read the header from `reader`.
    pub fn read_from<R>(mut reader: R) -> io::Result<Self>
    where
        R: Read,
    {
        let mut buf = vec![0u8; Self::SIZE];
        reader.read_exact(&mut buf)?;

        Ok(Self {
            size: u64::from_be_bytes(buf[0..8].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_read_write() {
        let h1 = Header { size: 42 };

        let mut buf = vec![0u8; Header::SIZE];
        h1.write_to(buf.as_mut_slice()).unwrap();

        let h2 = Header::read_from(buf.as_slice()).unwrap();

        assert_eq!(h1, h2);
    }
}
