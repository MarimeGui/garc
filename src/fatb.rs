use crate::Result;
use ez_io::{MagicNumberCheck, ReadE, WriteE};
use std::io::{Read, Write};

pub struct FATB {
    /// Size in bytes of this section.
    pub fatb_size: u32,
    /// Number of entries
    pub nb_entries: u32,
    /// Data entries
    pub entries: Vec<Entry>,
}

impl FATB {
    /// Reads the FATO data, should be seeked after the FATO section
    pub fn import<R: Read>(reader: &mut R) -> Result<FATB> {
        reader.check_magic_number(&[b'B', b'T', b'A', b'F'])?;
        let fatb_size = reader.read_le_to_u32()?;
        let nb_entries = reader.read_le_to_u32()?;
        let mut entries = Vec::with_capacity(nb_entries as usize);
        for _ in 0..nb_entries {
            entries.push(Entry::import(reader)?);
        }
        Ok(FATB {
            fatb_size,
            nb_entries,
            entries,
        })
    }

    /// Writes the FATO data, should be seeked after the FATO section
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'B', b'T', b'A', b'F'])?;
        writer.write_le_to_u32(self.fatb_size)?;
        writer.write_le_to_u32(self.nb_entries)?;
        for e in &self.entries {
            e.export(writer)?;
        }
        Ok(())
    }
}

/// An Entry inside the FATB section
pub struct Entry {
    pub bits: u32,
    pub start_offset: u32,
    pub end_offset: u32,
    pub length: u32,
}

impl Entry {
    pub fn import<R: Read>(reader: &mut R) -> Result<Entry> {
        let bits = reader.read_le_to_u32()?;
        let start_offset = reader.read_le_to_u32()?;
        let end_offset = reader.read_le_to_u32()?;
        let length = reader.read_le_to_u32()?;
        Ok(Entry {
            bits,
            start_offset,
            end_offset,
            length,
        })
    }

    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_le_to_u32(self.bits)?;
        writer.write_le_to_u32(self.start_offset)?;
        writer.write_le_to_u32(self.end_offset)?;
        writer.write_le_to_u32(self.length)?;
        Ok(())
    }
}
