extern crate ez_io;

pub mod error;

use crate::error::GARCError;
use std::io::{Read, Write};

type Result<T> = ::std::result::Result<T, GARCError>;

pub struct GARC {}

impl GARC {
    pub fn import<R: Read>(reader: &mut R) -> Result<GARC> {
        let garc = GARC {};
        Ok(garc)
    }
    pub fn export<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}
