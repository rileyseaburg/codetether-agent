use super::super::super::bit_reader::BitReader;

pub(super) fn previous(
    reader: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    total: usize,
) -> Result<(), String> {
    let value = *out
        .last()
        .ok_or_else(|| "zlib inflate: missing repeat length".to_string())?;
    let count = reader.read_bits(2)? as usize + 3;
    push(out, total, value, count)
}

pub(super) fn zero(
    reader: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    total: usize,
    base: usize,
    bits: u8,
) -> Result<(), String> {
    let count = reader.read_bits(bits)? as usize + base;
    push(out, total, 0, count)
}

fn push(out: &mut Vec<u8>, total: usize, value: u8, count: usize) -> Result<(), String> {
    if out.len() + count > total {
        return Err("zlib inflate: code lengths overrun".into());
    }
    for _ in 0..count {
        out.push(value);
    }
    Ok(())
}
