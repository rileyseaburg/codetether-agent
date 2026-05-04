//! Minimal parser for the AWS `vnd.amazon.eventstream` binary frame format.
//!
//! Each frame layout:
//!
//! ```text
//! +----------------------------------------------------------+
//! | total_len:       u32 BE                                  |
//! | headers_len:     u32 BE                                  |
//! | prelude_crc:     u32 BE  (ignored — TLS provides integrity)
//! | headers:         headers_len bytes                        |
//! | payload:         total_len - headers_len - 16 bytes       |
//! | message_crc:     u32 BE  (ignored)                        |
//! +----------------------------------------------------------+
//! ```
//!
//! Each header: `name_len u8`, `name utf8`, `value_type u8`, then a
//! type-specific value. We only need type `7` (UTF-8 string, `u16 BE` length).
//!
//! This implementation is intentionally minimal: CRC validation is skipped
//! (TLS already protects integrity), and only string-typed headers are
//! decoded since Bedrock Converse only emits those.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::eventstream::FrameBuffer;
//!
//! // Build a tiny frame: no headers, payload = "hi".
//! let payload = b"hi";
//! let total_len: u32 = 16 + payload.len() as u32;
//! let mut frame = Vec::new();
//! frame.extend_from_slice(&total_len.to_be_bytes());
//! frame.extend_from_slice(&0u32.to_be_bytes()); // headers_len
//! frame.extend_from_slice(&0u32.to_be_bytes()); // prelude_crc (ignored)
//! frame.extend_from_slice(payload);
//! frame.extend_from_slice(&0u32.to_be_bytes()); // message_crc (ignored)
//!
//! let mut buf = FrameBuffer::new();
//! buf.extend(&frame);
//! let msg = buf.next_frame().unwrap().expect("one frame");
//! assert_eq!(msg.payload, b"hi");
//! assert!(msg.headers.is_empty());
//! assert!(buf.next_frame().unwrap().is_none());
//! ```

use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// One decoded AWS eventstream message.
#[derive(Debug, Clone, Default)]
pub struct EventMessage {
    /// String-typed headers only; binary types are skipped.
    pub headers: HashMap<String, String>,
    /// Raw message payload (application-defined; typically JSON).
    pub payload: Vec<u8>,
}

impl EventMessage {
    /// Return the `:event-type` header (event variant name).
    pub fn event_type(&self) -> Option<&str> {
        self.headers.get(":event-type").map(String::as_str)
    }

    /// Return the `:message-type` header (`event`, `exception`, `error`).
    pub fn message_type(&self) -> Option<&str> {
        self.headers.get(":message-type").map(String::as_str)
    }
}

/// Streaming frame buffer: feed arbitrary byte chunks via [`FrameBuffer::extend`]
/// and pull out complete messages via [`FrameBuffer::next_frame`].
#[derive(Debug, Default)]
pub struct FrameBuffer {
    buf: Vec<u8>,
}

impl FrameBuffer {
    /// Create an empty buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append more bytes from the transport.
    pub fn extend(&mut self, chunk: &[u8]) {
        self.buf.extend_from_slice(chunk);
    }

    /// Try to parse the next full frame from the buffer.
    ///
    /// Returns:
    /// - `Ok(Some(msg))` — a complete message was decoded and removed.
    /// - `Ok(None)` — need more bytes.
    /// - `Err(_)` — malformed frame.
    pub fn next_frame(&mut self) -> Result<Option<EventMessage>> {
        if self.buf.len() < 12 {
            return Ok(None);
        }
        let total_len = u32::from_be_bytes(
            self.buf[0..4].try_into().map_err(|_| anyhow!("short read"))?
        ) as usize;
        let headers_len = u32::from_be_bytes(
            self.buf[4..8].try_into().map_err(|_| anyhow!("short read"))?
        ) as usize;

        if total_len < 16 || headers_len + 16 > total_len {
            return Err(anyhow!(
                "invalid eventstream frame: total_len={total_len}, headers_len={headers_len}"
            ));
        }
        if self.buf.len() < total_len {
            return Ok(None);
        }

        // prelude ends at 12; headers [12, 12+headers_len); payload
        // [12+headers_len, total_len-4); trailing CRC 4 bytes.
        let headers_start = 12usize;
        let headers_end = headers_start + headers_len;
        let payload_end = total_len - 4;

        let headers = parse_headers(&self.buf[headers_start..headers_end])?;
        let payload = self.buf[headers_end..payload_end].to_vec();

        self.buf.drain(..total_len);
        Ok(Some(EventMessage { headers, payload }))
    }
}

fn parse_headers(mut bytes: &[u8]) -> Result<HashMap<String, String>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_frame(headers: &[(&str, &str)], payload: &[u8]) -> Vec<u8> {
        let mut header_bytes = Vec::new();
        for (k, v) in headers {
            header_bytes.push(k.len() as u8);
            header_bytes.extend_from_slice(k.as_bytes());
            header_bytes.push(7u8); // string type
            header_bytes.extend_from_slice(&(v.len() as u16).to_be_bytes());
            header_bytes.extend_from_slice(v.as_bytes());
        }
        let total_len = 16 + header_bytes.len() + payload.len();
        let mut frame = Vec::new();
        frame.extend_from_slice(&(total_len as u32).to_be_bytes());
        frame.extend_from_slice(&(header_bytes.len() as u32).to_be_bytes());
        frame.extend_from_slice(&0u32.to_be_bytes());
        frame.extend_from_slice(&header_bytes);
        frame.extend_from_slice(payload);
        frame.extend_from_slice(&0u32.to_be_bytes());
        frame
    }

    #[test]
    fn parses_single_frame_with_headers() {
        let frame = build_frame(
            &[(":event-type", "messageStart"), (":message-type", "event")],
            br#"{"role":"assistant"}"#,
        );
        let mut buf = FrameBuffer::new();
        buf.extend(&frame);
        let msg = buf.next_frame().unwrap().unwrap();
        assert_eq!(msg.event_type(), Some("messageStart"));
        assert_eq!(msg.message_type(), Some("event"));
        assert_eq!(msg.payload, br#"{"role":"assistant"}"#);
    }

    #[test]
    fn handles_chunked_delivery() {
        let frame = build_frame(&[(":event-type", "x")], b"hello");
        let mut buf = FrameBuffer::new();
        buf.extend(&frame[..5]);
        assert!(buf.next_frame().unwrap().is_none());
        buf.extend(&frame[5..]);
        assert!(buf.next_frame().unwrap().is_some());
    }

    #[test]
    fn parses_multiple_frames() {
        let mut all = Vec::new();
        all.extend(build_frame(&[(":event-type", "a")], b"1"));
        all.extend(build_frame(&[(":event-type", "b")], b"22"));
        let mut buf = FrameBuffer::new();
        buf.extend(&all);
        assert_eq!(buf.next_frame().unwrap().unwrap().event_type(), Some("a"));
        assert_eq!(buf.next_frame().unwrap().unwrap().event_type(), Some("b"));
        assert!(buf.next_frame().unwrap().is_none());
    }
}
