use ez_io::error::{MagicNumberCheckError, WrongMagicNumber};
use std::io::Error as IOError;

#[derive(Debug)]
pub enum GARCError {
    IO(IOError),
    MagicNumber(WrongMagicNumber),
    HeaderTooSmall(u32),
    HeaderDataMismatch,
    NbEntriesMismatch(u16, u32),
    NoSuchIndex(usize),
    UnknownCompressionAlgorithm(u8),
}

impl From<IOError> for GARCError {
    fn from(e: IOError) -> GARCError {
        GARCError::IO(e)
    }
}

impl From<MagicNumberCheckError> for GARCError {
    fn from(e: MagicNumberCheckError) -> GARCError {
        match e {
            MagicNumberCheckError::IoError(ioe) => GARCError::IO(ioe),
            MagicNumberCheckError::MagicNumber(mne) => GARCError::MagicNumber(mne),
        }
    }
}
