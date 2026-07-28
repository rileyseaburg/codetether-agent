use super::{alphabet, clean};

pub(super) fn text(input: &str) -> Result<String, String> {
    let chars = clean::input(input)?;
    let bytes = decode_chars(&chars);
    Ok(bytes.into_iter().map(char::from).collect())
}

fn decode_chars(chars: &[char]) -> Vec<u8> {
    let (mut bits, mut bit_count, mut out) = (0u32, 0u8, Vec::new());
    for ch in chars {
        bits = (bits << 6) | alphabet::value(*ch).unwrap() as u32;
        bit_count += 6;
        while bit_count >= 8 {
            bit_count -= 8;
            out.push(((bits >> bit_count) & 0xff) as u8);
        }
    }
    out
}
