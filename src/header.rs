use crate::error::GARCError;
use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

/// Header of a GARC, excluding the 4-byte Magic Number. Some fields are optional.
pub struct Header {
    /// Size in bytes of the header, including the magic number and all fields described by this Struct
    pub header_size: u32,
    /// Indicates the Endianness of this file
    pub endianness: u16,
    /// Version of the file
    pub version: u16,
    pub unk_1: u32, // Seems to always be 4
    /// Absolute offset to the beginning of the compressed data inside the GARC file
    pub data_offset: u32,
    /// Total size of this file, should be the same as the size of the file where this is read from
    pub file_size: u32,
    pub unk_3: u32,         // Proportional to the file size
    pub unk_4: Option<u32>, // Proportional to the file size, sometimes equal to unk_3
    pub unk_5: Option<u32>, // Seems to always be 4
}

impl Header {
    /// Imports the data inside of the Header, should be seeked after the Magic Number
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Header> {
        let header_size = reader.read_le_to_u32()?;
        if header_size < 28 {
            return Err(GARCError::HeaderTooSmall(header_size));
        }
        let endianness = reader.read_le_to_u16()?;
        let version = reader.read_le_to_u16()?;
        let unk_1 = reader.read_le_to_u32()?;
        let data_offset = reader.read_le_to_u32()?;
        let file_size = reader.read_le_to_u32()?;
        let unk_3 = reader.read_le_to_u32()?;
        let unk_4 = if header_size >= 36 {
            Some(reader.read_le_to_u32()?)
        } else {
            None
        };
        let unk_5 = if header_size >= 36 {
            Some(reader.read_le_to_u32()?)
        } else {
            None
        };
        // If the header is bigger than expected, skip the data we can't read
        if header_size > 36 {
            reader.seek(SeekFrom::Current(i64::from(header_size - 36)))?;
        }
        Ok(Header {
            header_size,
            endianness,
            version,
            unk_1,
            data_offset,
            file_size,
            unk_3,
            unk_4,
            unk_5,
        })
    }

    /// Exports the data inside of the Header, should be seeked after the Magic Number
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        // Check for the size of the header along with the data present
        match (self.header_size, self.unk_4.is_some(), self.unk_5.is_some()) {
            (28, false, false) => {}
            (36, true, true) => {}
            _ => return Err(GARCError::HeaderDataMismatch),
        }
        writer.write_le_to_u32(self.header_size)?;
        writer.write_le_to_u16(self.endianness)?;
        writer.write_le_to_u16(self.version)?;
        writer.write_le_to_u32(self.unk_1)?;
        writer.write_le_to_u32(self.data_offset)?;
        writer.write_le_to_u32(self.file_size)?;
        writer.write_le_to_u32(self.unk_3)?;
        if let Some(v) = self.unk_4 {
            writer.write_le_to_u32(v)?
        }
        if let Some(v) = self.unk_5 {
            writer.write_le_to_u32(v)?
        }
        Ok(())
    }
}
