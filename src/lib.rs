extern crate ez_io;

pub mod compressed_header;
pub mod error;
pub mod fatb;
pub mod fato;
pub mod fimb;
pub mod header;
pub mod lz11;

use crate::compressed_header::CompressedHeader;
use crate::error::GARCError;
use crate::fatb::FATB;
use crate::fato::FATO;
use crate::fimb::FIMB;
use crate::header::Header;
use crate::lz11::decompress;
use ez_io::MagicNumberCheck;
use std::io::{Read, Seek, SeekFrom, Write};

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
        Ok(GARC {
            header,
            fato,
            fatb,
            fimb,
        })
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

    /// Extract a file from its index (GARC files do not have any filenames)
    pub fn extract<R: Read + Seek, W: Read + Write + Seek>(
        &self,
        reader: &mut R,
        writer: &mut W,
        file_index: usize,
    ) -> Result<()> {
        // Get the FATB entry
        let fatb_entry = match self.fatb.entries.get(file_index) {
            Some(o) => o,
            None => return Err(GARCError::NoSuchIndex(file_index)),
        };
        // Seek to the compressed data
        reader.seek(SeekFrom::Start(
            u64::from(self.header.data_offset) + u64::from(fatb_entry.start_offset),
        ))?;
        match fatb_entry.is_compressed() {
            true => {
                // Get compressed header
                let c_header = CompressedHeader::import(reader)?;
                // Check compression type
                if c_header.get_compression() != 0x11 {
                    return Err(GARCError::UnknownCompressionAlgorithm(
                        c_header.get_compression(),
                    ));
                }
                // Decompress the file
                decompress(reader, writer, c_header.get_decompressed_size() as usize)?;
            }
            false => {
                let mut buf = vec![0u8; fatb_entry.length as usize]; // Lossy
                reader.read_exact(&mut buf)?;
                writer.write_all(&buf)?;
            }
        }
        Ok(())
    }

    pub fn get_nb_files(&self) -> Result<u32> {
        // Check if the number of files is the same in FATO and FATB sections
        if u32::from(self.fato.nb_entries) != self.fatb.nb_entries {
            Err(GARCError::NbEntriesMismatch(
                self.fato.nb_entries,
                self.fatb.nb_entries,
            ))
        } else {
            Ok(self.fatb.nb_entries)
        }
    }
}
