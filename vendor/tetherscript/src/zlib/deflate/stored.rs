use super::BitReader;

pub(super) fn read(reader: &mut BitReader<'_>, out: &mut Vec<u8>) -> Result<(), String> {
    reader.align_byte();
    let len = read_u16(reader)?;
    let nlen = read_u16(reader)?;
    if len != !nlen {
        return Err("zlib inflate: invalid stored block length".into());
    }
    for _ in 0..len {
        out.push(reader.read_byte()?);
    }
    Ok(())
}

fn read_u16(reader: &mut BitReader<'_>) -> Result<u16, String> {
    let lo = reader.read_byte()? as u16;
    let hi = reader.read_byte()? as u16;
    Ok(lo | (hi << 8))
}
