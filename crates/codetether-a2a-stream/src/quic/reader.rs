//! Read length-delimited A2A frames from a QUIC stream with resume filtering.
//!
//! Frames whose `seq` is at or below the resume cursor's last-processed seq (for
//! the same epoch) are skipped, preserving the Phase 1 at-least-once delivery
//! contract when a stream is re-opened after migration or reconnect.

use super::super::frame::ParsedFrame;
use super::codec::decode_body;
use super::reader_outcome::ReadOneOutcome;
use anyhow::{Context, Result, anyhow};
use quinn::{ReadExactError, RecvStream};

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
    /// Returns `Ok(None)` only when the peer called `finish()` cleanly at a
    /// frame boundary (`ReadExactError::FinishedEarly` on the length prefix).
    /// Any other read error — connection reset, stream reset, truncation
    /// mid-body — is returned as `Err` so callers can distinguish a clean
    /// session end from an abnormal transport failure.
    ///
    /// Frames with no `data:` line (e.g. heartbeats) are skipped transparently.
    ///
    /// # Errors
    /// Returns an error on transport failure, truncated body, or invalid UTF-8.
    pub async fn next_frame(&mut self) -> Result<Option<ParsedFrame>> {
        loop {
            let ReadOneOutcome::Frame(frame) = self.read_one().await? else {
                return Ok(None);
            };
            // Skip heartbeat / no-data frames without ending the session.
            let Some(frame) = frame else { continue };
            if let (Some(floor), Some(id)) = (self.resume_seq, frame.id.as_ref())
                && id.seq <= floor
            {
                continue;
            }
            return Ok(Some(frame));
        }
    }

    pub(super) async fn read_one(&mut self) -> Result<ReadOneOutcome> {
        let mut len_buf = [0u8; 4];
        match self.recv.read_exact(&mut len_buf).await {
            Ok(()) => {}
            // Clean EOF at a frame boundary: the writer called finish().
            Err(ReadExactError::FinishedEarly(_)) => return Ok(ReadOneOutcome::Eof),
            // Anything else is a real transport error.
            Err(ReadExactError::ReadError(e)) => {
                return Err(anyhow!("quic stream read error: {e}"));
            }
        }
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; len];
        self.recv
            .read_exact(&mut body)
            .await
            .context("truncated quic frame body")?;
        let text = String::from_utf8(body).context("non-utf8 quic frame")?;
        // decode_body returns None for frames with no data: line (heartbeats).
        Ok(ReadOneOutcome::Frame(decode_body(&text)))
    }
}
