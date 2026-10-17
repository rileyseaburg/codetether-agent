//! dedup transform rules.
//! Content-addressed deduplication of tool-result blocks.
//!
//! Agentic loops re-read the same file and re-run the same searches many
//! times. Each duplicate tool output costs full input tokens on every
//! subsequent turn. This module detects exact content duplicates via
//! SHA-256 and replaces later copies with a short back-reference that
//! points at the first occurrence.
//!
//! # Safety
//!
//! *No information is lost* — the model can always ask the agent to
//! re-run the original tool call. The back-reference preserves the
//! `tool_call_id` of the first sighting so the model (or a human
//! auditing the transcript) can correlate the two.
//!
//! Only `ContentPart::ToolResult` blocks are considered; text,
//! assistant messages, tool-call arguments, images, and thinking blocks
//! are left untouched.
//!
//! # Threshold
//!
//! Tool outputs smaller than [`MIN_DEDUP_BYTES`] are left alone — the
//! marker itself would be nearly as long as the content, and small
//! outputs typically carry disambiguating structure (e.g. `"ok"` vs
//! `"error"`).

use sha2::{Digest, Sha256};

use std::collections::HashMap;

use super::ExperimentalStats;

use codetether_provider_core::{ContentPart, Message};

/// Tool outputs shorter than this byte count are never deduplicated.
/// Chosen so the back-reference marker is always shorter than the
/// content it replaces.
pub const MIN_DEDUP_BYTES: usize = 256;

/// Never deduplicate tool results in this many trailing messages. When
/// a user asks the agent to re-run an identical call (e.g. "try that
/// again"), the freshly-produced output would otherwise be replaced
/// with a back-reference to the previous sighting, making the retry
/// appear to produce no new information. Keeping the tail verbatim
/// lets the model see that the retry actually happened.
pub const KEEP_LAST_MESSAGES: usize = 8;
mod dedup_tool_outputs;
pub use dedup_tool_outputs::dedup_tool_outputs;
#[cfg(test)]
mod tests;
