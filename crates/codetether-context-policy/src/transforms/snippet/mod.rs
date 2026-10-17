//! snippet transform rules.
//! Head/tail snippet compaction of stale oversized tool outputs.
//!
//! Late in an agent loop, early-turn tool outputs are rarely referenced
//! but still cost full input tokens on every subsequent step. This
//! strategy replaces the *middle* of any tool-result block that is
//!
//! * older than [`RECENCY_WINDOW`] messages back, and
//! * larger than [`SNIPPET_THRESHOLD_BYTES`],
//!
//! with a `[...<N> bytes elided...]` marker, keeping a head and tail
//! window so the model can still see the shape of the output.
//!
//! # Safety
//!
//! Lossy but referenceable — the tool_call_id is preserved so the model
//! can request a re-run if it needs the full content. The head and tail
//! windows are generous enough (1 KB each by default) to surface error
//! messages, file headers, or the last lines of a log.
//!
//! # Rollout
//!
//! This strategy is intentionally kept out of the default-safe
//! [`super::apply_all`] path. It is useful under real token pressure,
//! but it is still lossy because it removes unique bytes from old tool
//! outputs.

use super::ExperimentalStats;

use codetether_provider_core::{ContentPart, Message};

/// Tool results in the last N messages are left alone — the model is
/// likely actively reasoning about them.
pub const RECENCY_WINDOW: usize = 8;

/// Tool outputs shorter than this are never snipped.
pub const SNIPPET_THRESHOLD_BYTES: usize = 4096;

/// Bytes kept at the start of a snipped output.
pub const HEAD_BYTES: usize = 1024;

/// Bytes kept at the end of a snipped output.
pub const TAIL_BYTES: usize = 1024;
mod snippet_stale_tool_outputs;
pub use snippet_stale_tool_outputs::snippet_stale_tool_outputs;
mod floor_char_boundary;
use floor_char_boundary::floor_char_boundary;
mod ceil_char_boundary;
use ceil_char_boundary::ceil_char_boundary;
#[cfg(test)]
mod tests;
