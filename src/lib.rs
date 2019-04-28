extern crate ez_io;

pub mod error;
pub mod fatb;
pub mod fato;
pub mod header;
pub mod fimb;

use crate::error::GARCError;
use crate::fatb::FATB;
use crate::fato::FATO;
use crate::fimb::FIMB;
use crate::header::Header;
use ez_io::MagicNumberCheck;
use std::io::{Read, Seek, Write};

type Result<T> = ::std::result::Result<T, GARCError>;

/// Main type used in this crate. This Struct allows for easy Import / Export of GARC files. This contains the data necessary for extracting the files, but it does not contain directly the binary data of the files.
pub struct GARC {
    /// Header containing general information about the file
    pub header: Header,
    pub fato: FATO,
    pub fatb: FATB,
    pub fimb: FIMB,
}

impl GARC {
    /// Reads an entire GARC file to memory
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<GARC> {
        reader.check_magic_number(&[b'C', b'R', b'A', b'G'])?;
        let header = Header::import(reader)?;
        let fato = FATO::import(reader)?;
        let fatb = FATB::import(reader)?;
        let fimb = FIMB::import(reader)?;
        Ok(GARC { header, fato, fatb, fimb })
    }

    /// Exports an entire GARC file from memory
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'C', b'R', b'A', b'G'])?;
        self.header.export(writer)?;
        self.fato.export(writer)?;
        self.fatb.export(writer)?;
        self.fimb.export(writer)?;
        Ok(())
    }
}
