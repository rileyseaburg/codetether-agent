//! Header iteration and value-type dispatch.
use anyhow::{Result, anyhow};
use std::collections::HashMap;
mod string;

pub(super) fn parse_headers(mut bytes: &[u8]) -> Result<HashMap<String, String>> {
    let mut out = HashMap::new();
    while !bytes.is_empty() {
        if bytes.is_empty() {
            break;
        }
        let name_len = bytes[0] as usize;
        bytes = &bytes[1..];
        if bytes.len() < name_len + 1 {
            return Err(anyhow!("truncated header name"));
        }
        let name = std::str::from_utf8(&bytes[..name_len])
            .map_err(|e| anyhow!("bad header name utf8: {e}"))?
            .to_string();
        bytes = &bytes[name_len..];
        let value_type = bytes[0];
        bytes = &bytes[1..];

        match value_type {
            // UTF-8 string, u16 BE length
            7 => {
                let (value, rest) = string::decode(bytes)?;
                bytes = rest;
                out.insert(name, value);
            }
            // Skip other types (bool, byte, int16/32/64, bytes, timestamp, uuid)
            0 | 1 => {} // true / false — no value bytes
            2 => bytes = &bytes[1..],
            3 => bytes = &bytes[2..],
            4 => bytes = &bytes[4..],
            5 => bytes = &bytes[8..],
            6 | 8 => {
                // byte array / timestamp - u16-prefixed or 8 bytes
                if value_type == 6 {
                    let vlen = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
                    bytes = &bytes[2 + vlen..];
                } else {
                    bytes = &bytes[8..];
                }
            }
            9 => bytes = &bytes[16..], // uuid
            _ => return Err(anyhow!("unknown header type {value_type}")),
        }
    }
    Ok(out)
}
