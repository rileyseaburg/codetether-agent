//! Length-prefixed UTF-8 header value decoding.
use anyhow::{Result, anyhow};
pub(super) fn decode(mut bytes: &[u8]) -> Result<(String, &[u8])> {
    if bytes.len() < 2 {
        return Err(anyhow!("truncated header value length"));
    }
    let vlen = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
    bytes = &bytes[2..];
    if bytes.len() < vlen {
        return Err(anyhow!("truncated header value"));
    }
    let value = std::str::from_utf8(&bytes[..vlen])
        .map_err(|e| anyhow!("bad header value utf8: {e}"))?
        .to_string();
    bytes = &bytes[vlen..];

    Ok((value, bytes))
}
