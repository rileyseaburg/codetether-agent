//! Canvas color parsing.

pub(super) fn parse(style: &str) -> [u8; 4] {
    let lower = style.trim().to_ascii_lowercase();
    match lower.as_str() {
        "transparent" => [0, 0, 0, 0],
        "white" => [255, 255, 255, 255],
        "red" => [255, 0, 0, 255],
        "green" => [0, 128, 0, 255],
        "blue" => [0, 0, 255, 255],
        _ => hex(&lower).unwrap_or([0, 0, 0, 255]),
    }
}

pub(super) fn safe_style(style: &str) -> String {
    style
        .chars()
        .map(|ch| {
            if matches!(ch, '|' | ';' | '\n' | '\r') {
                ' '
            } else {
                ch
            }
        })
        .collect()
}

fn hex(value: &str) -> Option<[u8; 4]> {
    let hex = value.strip_prefix('#')?;
    if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
        return Some([r, g, b, 255]);
    }
    if hex.len() == 6 {
        return Some([
            u8::from_str_radix(&hex[0..2], 16).ok()?,
            u8::from_str_radix(&hex[2..4], 16).ok()?,
            u8::from_str_radix(&hex[4..6], 16).ok()?,
            255,
        ]);
    }
    None
}
