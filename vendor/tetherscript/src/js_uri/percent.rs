const RESERVED: &[u8] = b";,/?:@&=+$#";

#[path = "percent/hex.rs"]
mod hex;

pub(super) fn decode(input: &str, keep_reserved: bool) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Some(byte) = hex::byte(bytes[i + 1], bytes[i + 2]) {
                if keep_reserved && RESERVED.contains(&byte) {
                    out.extend_from_slice(&bytes[i..i + 3]);
                } else {
                    out.push(byte);
                }
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

pub(super) fn encode(input: &str, keep_reserved: bool) -> String {
    let mut out = String::new();
    for byte in input.bytes() {
        if unescaped(byte) || (keep_reserved && RESERVED.contains(&byte)) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

fn unescaped(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
        )
}
