use super::BitReader;

const LENGTH_BASE: [usize; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [usize; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

pub(super) fn length(symbol: u16, reader: &mut BitReader<'_>) -> Result<usize, String> {
    let index = symbol
        .checked_sub(257)
        .filter(|index| *index < 29)
        .ok_or_else(|| "zlib inflate: invalid length symbol".to_string())?;
    Ok(LENGTH_BASE[index as usize] + reader.read_bits(LENGTH_EXTRA[index as usize])? as usize)
}

pub(super) fn distance(symbol: u16, reader: &mut BitReader<'_>) -> Result<usize, String> {
    let index = symbol as usize;
    if index >= DIST_BASE.len() {
        return Err("zlib inflate: invalid distance symbol".into());
    }
    Ok(DIST_BASE[index] + reader.read_bits(DIST_EXTRA[index])? as usize)
}
