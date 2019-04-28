use std::io::{Read, Write};
use crate::Result;
use ez_io::{ReadE, WriteE, MagicNumberCheck};

pub struct FIMB {
    pub fimb_size: u32,
    pub data_size: u32,
}

impl FIMB {
    pub fn import<R: Read>(reader: &mut R) -> Result<FIMB> {
        reader.check_magic_number(&[b'B', b'M', b'I', b'F'])?;
        let fimb_size = reader.read_le_to_u32()?;
        let data_size = reader.read_le_to_u32()?;
        Ok(FIMB {fimb_size, data_size})
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'B', b'M', b'I', b'F'])?;
        writer.write_le_to_u32(self.fimb_size)?;
        writer.write_le_to_u32(self.data_size)?;
        Ok(())
    }
}