use super::alphabet;

pub(super) fn text(input: &str) -> Result<String, String> {
    Ok(encode(&binary_bytes(input)?))
}

fn binary_bytes(input: &str) -> Result<Vec<u8>, String> {
    input
        .chars()
        .map(|ch| {
            let code = ch as u32;
            u8::try_from(code).map_err(|_| format!("btoa: U+{code:04X} is outside 0..255"))
        })
        .collect()
}

fn encode(bytes: &[u8]) -> String {
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        out.push(alphabet::TABLE[(b0 >> 2) as usize] as char);
        out.push(alphabet::TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        out.push(third(chunk, b1, b2));
        out.push(fourth(chunk, b2));
    }
    out
}

fn third(chunk: &[u8], b1: u8, b2: u8) -> char {
    match chunk.len() > 1 {
        true => alphabet::TABLE[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char,
        false => '=',
    }
}

fn fourth(chunk: &[u8], b2: u8) -> char {
    match chunk.len() > 2 {
        true => alphabet::TABLE[(b2 & 0x3f) as usize] as char,
        false => '=',
    }
}
