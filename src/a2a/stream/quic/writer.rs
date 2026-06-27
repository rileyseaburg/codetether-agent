//! Write length-delimited A2A frames to a QUIC stream.
//!
//! The counterpart to [`super::reader::QuicFrameReader`]: each frame is sent as
//! a `u32` length prefix followed by its serialized body, and `seq` is recorded
//! so the server can drive the resume floor on the next stream open.

use super::codec::encode_frame;
use super::super::frame::ParsedFrame;
use anyhow::{Context, Result};
use quinn::SendStream;

/// Writes framed events to a QUIC [`SendStream`].
pub struct QuicFrameWriter {
    send: SendStream,
    last_seq: Option<u64>,
}

impl QuicFrameWriter {
    /// Wrap a send stream for framed writes.
    pub fn new(send: SendStream) -> Self {
        Self {
            send,
            last_seq: None,
        }
    }

    /// The highest `seq` written so far, if any.
    pub fn last_seq(&self) -> Option<u64> {
        self.last_seq
    }

    /// Write one frame as a length-prefixed block.
    ///
    /// # Errors
    /// Returns an error if the underlying QUIC stream write fails.
    pub async fn write_frame(&mut self, frame: &ParsedFrame) -> Result<()> {
        let wire = encode_frame(frame);
        self.send
            .write_all(&wire)
            .await
            .context("write quic frame")?;
        if let Some(id) = &frame.id {
            self.last_seq = Some(id.seq);
        }
        Ok(())
    }

    /// Signal clean end-of-stream to the peer.
    ///
    /// # Errors
    /// Returns an error if the stream cannot be finished.
    pub fn finish(&mut self) -> Result<()> {
        self.send.finish().context("finish quic stream")
    }
}
