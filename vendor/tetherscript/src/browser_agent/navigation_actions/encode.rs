//! Form URL encoding helpers.

pub(crate) fn pairs(entries: &[(String, String)]) -> String {
    entries
        .iter()
        .map(|(name, value)| format!("{}={}", component(name), component(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn component(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
