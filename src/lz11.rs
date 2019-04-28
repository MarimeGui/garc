use crate::Result;
use ez_io::ReadE;
use std::io::{Read, Seek, SeekFrom, Write};

fn bits(b: u8) -> [bool; 8] {
    return [
        ((b >> 7) & 1) != 0,
        ((b >> 6) & 1) != 0,
        ((b >> 5) & 1) != 0,
        ((b >> 4) & 1) != 0,
        ((b >> 3) & 1) != 0,
        ((b >> 2) & 1) != 0,
        ((b >> 1) & 1) != 0,
        (b & 1) != 0,
    ];
}

// Straight from https://github.com/magical/nlzss/blob/f27414f373eab53bfe3c1a819c40eb800323e690/lzss3.py#L72
pub fn decompress<R: Read, W: Read + Write + Seek>(
    reader: &mut R,
    writer: &mut W,
    decompressed_size: usize,
) -> Result<()> {
    let mut bytes_written = 0usize;
    let mut b;
    while bytes_written < decompressed_size {
        b = reader.read_to_u8()?;
        for flag in bits(b).iter() {
            match flag {
                false => {
                    writer.write_all(&[reader.read_to_u8()?])?;
                    bytes_written += 1;
                }
                true => {
                    b = reader.read_to_u8()?;
                    let indicator = b >> 4;

                    let mut count: u32;

                    if indicator == 0 {
                        count = u32::from(b << 4);
                        b = reader.read_to_u8()?;
                        count += u32::from(b >> 4);
                        count += 0x11;
                    } else if indicator == 1 {
                        count = u32::from((b & 0xF) << 12) + u32::from(reader.read_to_u8()? << 4);
                        b = reader.read_to_u8()?;
                        count += u32::from(b >> 4);
                        count += 0x111;
                    } else {
                        count = u32::from(indicator);
                        count += 1;
                    }

                    let disp = ((b & 0xF) << 8) + reader.read_to_u8()? + 1;

                    writer.seek(SeekFrom::Current(-i64::from(disp)))?;
                    let to_write = writer.read_to_u8()?;
                    writer.seek(SeekFrom::Current(i64::from(disp) - 1))?;

                    let mut to_write_vec = Vec::with_capacity(count as usize);
                    for _ in 0..count {
                        to_write_vec.push(to_write);
                    }
                    writer.write_all(&to_write_vec)?;
                    bytes_written += to_write_vec.len();
                }
            }
            if decompressed_size <= bytes_written {
                break;
            }
        }
    }
    Ok(())
}
