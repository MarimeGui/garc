use crate::Result;
use ez_io::{ReadE, WriteE};
use std::io::{Read, Seek, SeekFrom, Write};

fn bits(b: u8) -> [bool; 8] {
    [
        ((b >> 7) & 1) != 0,
        ((b >> 6) & 1) != 0,
        ((b >> 5) & 1) != 0,
        ((b >> 4) & 1) != 0,
        ((b >> 3) & 1) != 0,
        ((b >> 2) & 1) != 0,
        ((b >> 1) & 1) != 0,
        (b & 1) != 0,
    ]
}

// Not so straight from https://github.com/magical/nlzss/blob/f27414f373eab53bfe3c1a819c40eb800323e690/lzss3.py#L72
pub fn decompress<R: Read, W: Read + Write + Seek>(
    reader: &mut R,
    writer: &mut W,
    decompressed_size: usize,
) -> Result<()> {
    let mut bytes_written = 0usize;
    while bytes_written < decompressed_size {
        for flag in bits(reader.read_to_u8()?).iter() {
            match flag {
                false => {
                    writer.write_all(&[reader.read_to_u8()?])?;
                    bytes_written += 1;
                }
                true => {
                    let byte1: u32 = u32::from(reader.read_to_u8()?);
                    let byte2: u32 = u32::from(reader.read_to_u8()?);
                    let byte3: u32;
                    let byte4: u32;

                    let count: u32;
                    let disp: u32;

                    match byte1 >> 4 {
                        0 => {
                            byte3 = u32::from(reader.read_to_u8()?);
                            count = (((byte1 & 0x0F) << 4) | (byte2 >> 4)) + 0x11;
                            disp = (((byte2 & 0x0F) << 8) | byte3) + 0x1;
                        }
                        1 => {
                            byte3 = u32::from(reader.read_to_u8()?);
                            byte4 = u32::from(reader.read_to_u8()?);
                            count = (((byte1 & 0x0F) << 12) | (byte2 << 4) | (byte3 >> 4)) + 0x111;
                            disp = (((byte3 & 0x0F) << 8) | byte4) + 0x1;
                        }
                        _ => {
                            count = ((byte1 & 0xF0) >> 4) + 0x1;
                            disp = (((byte1 & 0x0F) << 8) | byte2) + 0x1;
                        }
                    }

                    for _ in 0..count {
                        let write_head = writer.seek(SeekFrom::Current(0))?;
                        writer.seek(SeekFrom::Current(-i64::from(disp)))?;
                        let to_copy = writer.read_to_u8()?;
                        writer.seek(SeekFrom::Start(write_head))?;
                        writer.write_to_u8(to_copy)?;
                        bytes_written += 1;
                    }
                }
            }
            if decompressed_size <= bytes_written {
                break;
            }
        }
    }
    Ok(())
}
