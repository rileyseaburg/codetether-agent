//! tool call dedup transform rules.
//! Collapse redundant identical tool calls.
//!
//! When an agent re-issues the same tool call with the same arguments
//! (e.g. `read_file { path: "src/lib.rs" }` called at step 3 and again
//! at step 17), the *call* itself is wasted tokens — the earlier
//! response is already in history (and by this point usually already
//! deduplicated by [`super::dedup`]).
//!
//! This pass walks the buffer, hashes each `ContentPart::ToolCall` by
//! `(name, arguments)`, and replaces the *arguments* of later
//! duplicates with a short `[REPEAT of call_X]` marker. The tool's
//! result block for the repeat is left untouched so the model still
//! sees a consistent call→result pairing; dedup will have collapsed
//! the result body if the output was identical.
//!
//! # Safety
//!
//! * Only the **arguments** string is rewritten. Name, id, and
//!   thought_signature are preserved so every provider adapter still
//!   round-trips.
//! * Tool calls in the most recent [`KEEP_LAST_MESSAGES`] messages are
//!   never touched — the model is likely actively reasoning over them.
//! * Only triggers when arguments exceed [`MIN_COLLAPSE_BYTES`] so
//!   micro-arguments like `{}` stay readable.
//!
//! # Rollout
//!
//! This strategy is intentionally kept out of the default-safe
//! [`super::apply_all`] path because replacing historical arguments
//! with back-references can erase still-relevant call details.

use sha2::{Digest, Sha256};

use std::collections::HashMap;

use super::ExperimentalStats;

use codetether_provider_core::{ContentPart, Message};

/// Keep tool-call arguments verbatim in this many trailing messages.
pub const KEEP_LAST_MESSAGES: usize = 8;

/// Tool-call arguments shorter than this are never collapsed.
pub const MIN_COLLAPSE_BYTES: usize = 64;
mod collapse_duplicate_calls;
pub use collapse_duplicate_calls::collapse_duplicate_calls;
#[cfg(test)]
mod tests;
