use super::{BitReader, Huffman};

#[path = "dynamic/repeat.rs"]
mod repeat;

const ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

pub(super) fn trees(reader: &mut BitReader<'_>) -> Result<(Huffman, Huffman), String> {
    let hlit = reader.read_bits(5)? as usize + 257;
    let hdist = reader.read_bits(5)? as usize + 1;
    let hclen = reader.read_bits(4)? as usize + 4;
    let mut code_lengths = vec![0u8; 19];
    for slot in ORDER.iter().take(hclen) {
        code_lengths[*slot] = reader.read_bits(3)? as u8;
    }
    let code_tree = Huffman::new(&code_lengths)?;
    let lengths = lengths(reader, &code_tree, hlit + hdist)?;
    let lit = Huffman::new(&lengths[..hlit])?;
    let dist = Huffman::new(&lengths[hlit..])?;
    Ok((lit, dist))
}

fn lengths(reader: &mut BitReader<'_>, tree: &Huffman, total: usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(total);
    while out.len() < total {
        match tree.decode(reader)? {
            value @ 0..=15 => out.push(value as u8),
            16 => repeat::previous(reader, &mut out, total)?,
            17 => repeat::zero(reader, &mut out, total, 3, 3)?,
            18 => repeat::zero(reader, &mut out, total, 11, 7)?,
            _ => return Err("zlib inflate: invalid code length symbol".into()),
        }
    }
    Ok(out)
}
