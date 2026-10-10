//! Bounded OpenAI-style SSE parser: CRLF, split UTF-8, explicit `[DONE]`.
use anyhow::{Result, bail};

mod emit;

const LIMIT: usize = 131_072;

/// Incremental parser; feed bytes, receive content deltas.
///
/// ```
/// use codetether_companion_relay::SseParser;
/// let mut parser = SseParser::default();
/// let mut out = String::new();
/// let done = parser.feed(b"data: {\"choices\":[{\"delta\":{\"content\":\"hi\"}}]}\r\n\r\ndata: [DONE]\n\n",
///     &mut |t| out.push_str(t)).unwrap();
/// assert!(done);
/// assert_eq!(out, "hi");
/// ```
#[derive(Default)]
pub struct SseParser {
    pending: Vec<u8>,
    data: Vec<String>,
}
impl SseParser {
    /// Consume bytes; returns `true` once `[DONE]` is seen.
    ///
    /// # Errors
    /// Oversized events, provider errors, tool calls, or invalid JSON/UTF-8.
    pub fn feed(&mut self, chunk: &[u8], delta: &mut dyn FnMut(&str)) -> Result<bool> {
        self.pending.extend_from_slice(chunk);
        if self.pending.len() > LIMIT {
            bail!("Analysis event exceeds limit");
        }
        while let Some(at) = self.pending.iter().position(|b| *b == b'\n') {
            let raw: Vec<u8> = self.pending.drain(..=at).collect();
            let line = std::str::from_utf8(&raw)?
                .trim_end_matches('\n')
                .trim_end_matches('\r');
            if let Some(data) = line.strip_prefix("data:") {
                self.data.push(data.trim_start().to_string());
            }
            if self.data.iter().map(String::len).sum::<usize>() > LIMIT {
                bail!("Analysis event exceeds limit");
            }
            if !line.is_empty() || self.data.is_empty() {
                continue;
            }
            let payload = std::mem::take(&mut self.data).join("\n");
            if payload == "[DONE]" {
                return Ok(true);
            }
            emit::emit(&serde_json::from_str(&payload)?, delta)?;
        }
        Ok(false)
    }
}
