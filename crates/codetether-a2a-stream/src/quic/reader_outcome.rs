//! Internal outcome type for one `read_one` call in [`super::reader`].

use super::super::frame::ParsedFrame;

/// Result of one raw frame read from the QUIC stream.
pub(super) enum ReadOneOutcome {
    /// A frame body was read; inner `None` means no `data:` line (skip it).
    Frame(Option<ParsedFrame>),
    /// Clean end-of-stream: the writer called `finish()` at a frame boundary.
    Eof,
}
