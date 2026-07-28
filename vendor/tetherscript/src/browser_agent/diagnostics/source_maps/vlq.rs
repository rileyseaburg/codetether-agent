//! Source-map base64 VLQ decoder.

pub fn decode(segment: &str) -> Option<Vec<i64>> {
    let mut values = Vec::new();
    let mut value = 0;
    let mut shift = 0;
    for ch in segment.chars() {
        let digit = base64(ch)?;
        value += (digit & 31) << shift;
        if digit & 32 == 0 {
            values.push(signed(value));
            value = 0;
            shift = 0;
        } else {
            shift += 5;
        }
    }
    (shift == 0).then_some(values)
}

fn signed(value: i64) -> i64 {
    let magnitude = value >> 1;
    if value & 1 == 0 {
        magnitude
    } else {
        -magnitude
    }
}

fn base64(ch: char) -> Option<i64> {
    match ch {
        'A'..='Z' => Some((ch as u8 - b'A') as i64),
        'a'..='z' => Some((ch as u8 - b'a' + 26) as i64),
        '0'..='9' => Some((ch as u8 - b'0' + 52) as i64),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}
