use super::super::{bit_reader::BitReader, huffman::Huffman};
use super::tables;

pub(super) fn inflate(
    reader: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    lit: &Huffman,
    dist: &Huffman,
) -> Result<(), String> {
    loop {
        match lit.decode(reader)? {
            byte @ 0..=255 => out.push(byte as u8),
            256 => return Ok(()),
            symbol => copy_match(reader, out, dist, symbol)?,
        }
    }
}

fn copy_match(
    reader: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    dist: &Huffman,
    symbol: u16,
) -> Result<(), String> {
    let length = tables::length(symbol, reader)?;
    let distance = tables::distance(dist.decode(reader)?, reader)?;
    for _ in 0..length {
        let index = out
            .len()
            .checked_sub(distance)
            .ok_or_else(|| "zlib inflate: invalid distance".to_string())?;
        out.push(out[index]);
    }
    Ok(())
}
