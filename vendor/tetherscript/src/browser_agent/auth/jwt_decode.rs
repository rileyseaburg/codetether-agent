//! Zero-dependency JWT decoding for agent auth inspection.

use super::session_state::TokenClaim;

/// Decode a JWT token without verifying the signature.
/// Extracts iss, sub, exp claims from the payload.
pub fn decode_jwt(kind: &str, token: &str) -> Option<TokenClaim> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 { return None; }
    let header = String::from_utf8(base64_url_decode(parts[0])?).ok()?;
    let payload = String::from_utf8(base64_url_decode(parts[1])?).ok()?;
    Some(TokenClaim {
        kind: kind.to_string(),
        issuer: json_string(&payload, "iss"),
        subject: json_string(&payload, "sub"),
        expires_at: json_i64(&payload, "exp"),
        raw_header: header,
        raw_payload: payload,
    })
}

pub fn decode_from_cookie(name: &str, value: &str) -> Option<TokenClaim> {
    decode_jwt(&format!("cookie:{name}"), value)
}

pub fn decode_from_authorization(value: &str) -> Option<TokenClaim> {
    value.strip_prefix("Bearer ").and_then(|v| decode_jwt("authorization", v.trim()))
}

fn base64_url_decode(input: &str) -> Option<Vec<u8>> {
    let (mut bits, mut n, mut out) = (0u32, 0u8, Vec::new());
    for b in input.bytes() {
        let v = match b {
            b'A'..=b'Z' => b - b'A', b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52, b'-' => 62, b'_' => 63, b'=' => break,
            _ => return None,
        } as u32;
        bits = (bits << 6) | v; n += 6;
        if n >= 8 { n -= 8; out.push((bits >> n) as u8); bits &= (1 << n) - 1; }
    }
    Some(out)
}

fn json_string(s: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let i = s.find(&pat)? + pat.len();
    let r = s[i..].trim_start().strip_prefix(':')?.trim_start().strip_prefix('"')?;
    let end = r.find('"')?;
    Some(r[..end].to_string())
}

fn json_i64(s: &str, key: &str) -> Option<i64> {
    let pat = format!("\"{key}\"");
    let i = s.find(&pat)? + pat.len();
    let r = s[i..].trim_start().strip_prefix(':')?.trim_start();
    let end = r.find(|c: char| !c.is_ascii_digit()).unwrap_or(r.len());
    r[..end].parse().ok()
}
