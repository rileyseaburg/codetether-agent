//! Read length-delimited A2A frames from a QUIC stream with resume filtering.
//!
//! Frames whose `seq` is at or below the resume cursor's last-processed seq (for
//! the same epoch) are skipped, preserving the Phase 1 at-least-once delivery
//! contract when a stream is re-opened after migration or reconnect.

use super::codec::decode_body;
use super::super::frame::ParsedFrame;
use anyhow::{Context, Result};
use quinn::RecvStream;

/// Reads framed events from a QUIC [`RecvStream`], honoring a resume floor.
pub struct QuicFrameReader {
    recv: RecvStream,
    resume_seq: Option<u64>,
}

impl QuicFrameReader {
    /// Build a reader that skips frames at or below `resume_seq`.
    pub fn new(recv: RecvStream, resume_seq: Option<u64>) -> Self {
        Self { recv, resume_seq }
    }

    /// Read the next deliverable frame, or `None` at clean end-of-stream.
    ///
    /// # Errors
    /// Returns an error on a truncated prefix/body or invalid UTF-8.
    pub async fn next_frame(&mut self) -> Result<Option<ParsedFrame>> {
        loop {
            let Some(frame) = self.read_one().await? else {
                return Ok(None);
            };
            if let (Some(floor), Some(id)) = (self.resume_seq, frame.id.as_ref())
                && id.seq <= floor
            {
                continue;
            }
            return Ok(Some(frame));
        }
    }

    async fn read_one(&mut self) -> Result<Option<ParsedFrame>> {
        let mut len_buf = [0u8; 4];
        match self.recv.read_exact(&mut len_buf).await {
            Ok(()) => {}
            Err(_) => return Ok(None),
        }
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; len];
        self.recv
            .read_exact(&mut body)
            .await
            .context("truncated quic frame body")?;
        let text = String::from_utf8(body).context("non-utf8 quic frame")?;
        Ok(decode_body(&text))
    }
}
