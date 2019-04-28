use crate::Result;
use ez_io::ReadE;
use std::io::{Read, Write};

pub struct CompressedHeader {
    pub raw: [u8; 4],
    pub extra_decompressed_size: Option<[u8; 4]>,
}

impl CompressedHeader {
    pub fn import<R: Read>(reader: &mut R) -> Result<CompressedHeader> {
        let mut raw = [0u8; 4];
        reader.read_exact(&mut raw)?;
        let extra_decompressed_size = if reversed_24bits_to_u32([raw[1], raw[2], raw[3]]) == 0 {
            let mut bytes = [0u8; 4];
            reader.read_exact(&mut bytes)?;
            Some(bytes)
        } else {
            None
        };
        Ok(CompressedHeader {
            raw,
            extra_decompressed_size,
        })
    }

    // pub fn export<W: Write>(reader: &mut W) -> Result<()> {
    //     Ok(())
    // }

    pub fn get_decompressed_size(&self) -> u32 {
        match self.extra_decompressed_size {
            Some(extra) => reversed_32bits_to_u32(extra),
            None => reversed_24bits_to_u32([self.raw[1], self.raw[2], self.raw[3]]),
        }
    }

    pub fn get_compression(&self) -> u8 {
        self.raw[0]
    }
}

pub fn reversed_24bits_to_u32(bytes: [u8; 3]) -> u32 {
    u32::from(bytes[0]) | u32::from(bytes[1]) << 8 | u32::from(bytes[2]) << 16
}

pub fn reversed_32bits_to_u32(bytes: [u8; 4]) -> u32 {
    u32::from(bytes[0])
        | u32::from(bytes[1]) << 8
        | u32::from(bytes[2]) << 16
        | u32::from(bytes[3]) << 24
}
