use std::error::Error as StdError;
use std::string::FromUtf16Error;

use core2::io::{self, Read, Seek, Write};

/// UTF-16 Little Endian **without** a byte-order marker (BOM).
pub struct Utf16Le;

/// UTF-16 Big Endian **without** a byte-order marker (BOM).
pub struct Utf16Be;

/// UTF-16 Little Endian **without** a byte-order marker (BOM).
pub struct Utf16LeBom;

/// UTF-16 Big Endian **with** a byte-order marker (BOM).
pub struct Utf16BeBom;

pub const BOM_LE_BYTES: [u8; 2] = [0xFF, 0xFE];
pub const BOM_BE_BYTES: [u8; 2] = [0xFE, 0xFF];
pub const BOM_LE: u16 = 0xFFFE;
pub const BOM_BE: u16 = 0xFEFF;

pub trait Utf16Ext {
    type Error: StdError;

    fn encode_utf16_le(&self) -> Vec<u8>;
    fn encode_utf16_le_bom(&self) -> Vec<u8>;
    fn encode_utf16_be(&self) -> Vec<u8>;
    fn encode_utf16_be_bom(&self) -> Vec<u8>;

    fn decode_utf16_le(&self, input: &[u8]) -> Result<String, Self::Error>;
    fn decode_utf16_le_bom(&self, input: &[u8]) -> Result<String, Self::Error>;
    fn decode_utf16_be(&self, input: &[u8]) -> Result<String, Self::Error>;
    fn decode_utf16_be_bom(&self, input: &[u8]) -> Result<String, Self::Error>;
}

pub struct Utf16Writer<W: io::Write> {
    // mode: Utf16,
    writer: W,
}

pub struct Utf16Reader<R: io::Read> {
    // mode: Utf16,
    reader: R,
}

impl<R: io::Read> Read for Utf16Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl<W: io::Write> Write for Utf16Writer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<S: io::Seek + io::Read> Seek for Utf16Reader<S> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        todo!()
    }
}

impl<S: io::Seek + io::Write> Seek for Utf16Writer<S> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeUtf16Error {
    #[error("Could not decode UTF-16 input")]
    Decode(#[from] FromUtf16Error),

    #[error("Input has invalid length to be a valid UTF-16 input.")]
    BadLength,
}

impl Utf16Ext for &str {
    type Error = DecodeUtf16Error;

    fn encode_utf16_le(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_utf16_le_bom(&self) -> Vec<u8> {
        // Take a rough shot at getting a sensible capacity
        let mut vec = Vec::with_capacity(self.len() * 2);

        for b in BOM_LE_BYTES {
            vec.push(b);
        }

        self.encode_utf16().for_each(|w| {
            for b in w.to_le_bytes() {
                vec.push(b);
            }
        });

        vec
    }

    fn encode_utf16_be(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_utf16_be_bom(&self) -> Vec<u8> {
        todo!()
    }

    fn decode_utf16_le(&self, input: &[u8]) -> Result<String, Self::Error> {
        if input.len() % 2 != 0 {
            return Err(DecodeUtf16Error::BadLength);
        }

        let bytes = input
            .chunks(2)
            .map(|x| u16::from_le_bytes([x[0], x[1]]))
            .collect::<Vec<u16>>();

        Ok(String::from_utf16(&bytes)?)
    }

    fn decode_utf16_le_bom(&self, input: &[u8]) -> Result<String, Self::Error> {
        todo!()
    }

    fn decode_utf16_be(&self, input: &[u8]) -> Result<String, Self::Error> {
        todo!()
    }

    fn decode_utf16_be_bom(&self, input: &[u8]) -> Result<String, Self::Error> {
        todo!()
    }
}

impl Utf16Ext for String {
    type Error = DecodeUtf16Error;

    fn encode_utf16_le(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_utf16_le_bom(&self) -> Vec<u8> {
        <&str as Utf16Ext>::encode_utf16_le_bom(&&**self)
    }

    fn encode_utf16_be(&self) -> Vec<u8> {
        todo!()
    }

    fn encode_utf16_be_bom(&self) -> Vec<u8> {
        todo!()
    }

    fn decode_utf16_le(&self, input: &[u8]) -> Result<String, Self::Error> {
        <&str as Utf16Ext>::decode_utf16_le(&&**self, input)
    }

    fn decode_utf16_le_bom(&self, input: &[u8]) -> Result<String, Self::Error> {
        todo!()
    }

    fn decode_utf16_be(&self, input: &[u8]) -> Result<String, Self::Error> {
        todo!()
    }

    fn decode_utf16_be_bom(&self, input: &[u8]) -> Result<String, Self::Error> {
        todo!()
    }
}
