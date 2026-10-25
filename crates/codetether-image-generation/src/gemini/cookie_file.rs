//! UTF-8 and Windows UTF-16LE Netscape export decoding.
use anyhow::{Result, bail};

pub(super) fn decode(bytes: &[u8]) -> Result<String> {
    if bytes.len() > 1024 * 1024 {
        bail!("Gemini cookie export exceeds size limit");
    }
    let decoded = if let Some(body) = bytes.strip_prefix(&[0xff, 0xfe]) {
        if body.len() % 2 != 0 {
            bail!("Invalid UTF-16 cookie export");
        }
        let units: Vec<_> = body
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();
        String::from_utf16(&units).map_err(|_| anyhow::anyhow!("Invalid UTF-16 cookie export"))?
    } else {
        String::from_utf8(bytes.to_vec())
            .map_err(|_| anyhow::anyhow!("Invalid UTF-8 cookie export"))?
    };
    Ok(decoded.trim_start_matches('\u{feff}').to_owned())
}
