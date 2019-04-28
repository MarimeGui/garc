extern crate ez_io;

pub mod error;
pub mod fato;
pub mod header;

use crate::error::GARCError;
use crate::fato::FATO;
use crate::header::Header;
use ez_io::MagicNumberCheck;
use std::io::{Read, Seek, Write};

type Result<T> = ::std::result::Result<T, GARCError>;

/// Main type used in this crate. This Struct allows for easy Import / Export of GARC files.
pub struct GARC {
    /// Header containing general information about the file
    pub header: Header,
    pub fato: FATO,
}

impl GARC {
    /// Reads an entire GARC file to memory
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<GARC> {
        reader.check_magic_number(&[b'C', b'R', b'A', b'G'])?;
        let header = Header::import(reader)?;
        let fato = FATO::import(reader)?;
        Ok(GARC { header, fato })
    }

    /// Exports an entire GARC file from memory
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'C', b'R', b'A', b'G'])?;
        self.header.export(writer)?;
        self.fato.export(writer)?;
        Ok(())
    }
}
