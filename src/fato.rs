use crate::Result;
use ez_io::{MagicNumberCheck, ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct FATO {
    /// Size in bytes of this section.
    pub fato_size: u32,
    /// Number of entries
    pub nb_entries: u16,
    /// Data entries
    pub data: Vec<u32>,
}

impl FATO {
    /// Reads the FATO data, should be seeked after the Header
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<FATO> {
        reader.check_magic_number(&[b'O', b'T', b'A', b'F'])?;
        let fato_size = reader.read_le_to_u32()?;
        let nb_entries = reader.read_le_to_u16()?;
        reader.seek(SeekFrom::Current(2))?;
        let mut data = Vec::with_capacity(nb_entries as usize);
        for _ in 0..nb_entries {
            data.push(reader.read_le_to_u32()?);
        }
        Ok(FATO {
            fato_size,
            nb_entries,
            data,
        })
    }

    /// Writes the FATO data, should be seeked after the Header
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'O', b'T', b'A', b'F'])?;
        writer.write_le_to_u32(self.fato_size)?;
        writer.write_le_to_u16(self.nb_entries)?;
        writer.write_all(&[0xFF, 0xFF])?;
        for v in &self.data {
            writer.write_le_to_u32(*v)?;
        }
        Ok(())
    }
}
