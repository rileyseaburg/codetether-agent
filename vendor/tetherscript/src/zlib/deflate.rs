use super::{bit_reader::BitReader, huffman::Huffman};

#[path = "deflate/codes.rs"]
mod codes;
#[path = "deflate/dynamic.rs"]
mod dynamic;
#[path = "deflate/fixed.rs"]
mod fixed;
#[path = "deflate/stored.rs"]
mod stored;
#[path = "deflate/tables.rs"]
mod tables;

pub(super) fn inflate(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut reader = BitReader::new(data);
    let mut out = Vec::new();
    loop {
        let final_block = reader.read_bits(1)? == 1;
        match reader.read_bits(2)? {
            0 => stored::read(&mut reader, &mut out)?,
            1 => {
                let (lit, dist) = fixed::trees()?;
                codes::inflate(&mut reader, &mut out, &lit, &dist)?;
            }
            2 => {
                let (lit, dist) = dynamic::trees(&mut reader)?;
                codes::inflate(&mut reader, &mut out, &lit, &dist)?;
            }
            _ => return Err("zlib inflate: reserved deflate block type".into()),
        }
        if final_block {
            return Ok(out);
        }
    }
}
