extern crate ez_io;

pub mod error;

use crate::error::GARCError;
use std::io::Read;

type Result<T> = ::std::result::Result<T, GARCError>;

pub struct GARC {}

impl GARC {
    pub fn import<R: Read>(reader: &mut R) -> Result<GARC> {
        let garc = GARC {};
        Ok(garc)
    }
}
