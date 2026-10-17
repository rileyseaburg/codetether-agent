//! lingua transform rules.
//! Heuristic LLMLingua-style token/line pruning on stale assistant text.
//!
//! Inspired by *LLMLingua* and *LLMLingua-2* (Jiang et al., 2023-2024),
//! which use a small LM to score tokens by importance and drop the
//! low-information tail. We cannot ship a small LM in-process, so this
//! module applies a **heuristic** subset of the same idea — the parts
//! that are known-safe and require no model:
//!
//! * Collapse runs of 3+ blank lines to a single blank.
//! * Drop lines that are pure whitespace, punctuation, or ellipses.
//! * Collapse `" "`+ (runs of spaces) inside text to a single space,
//!   except inside fenced code blocks.
//! * Strip trailing whitespace from every line.
//!
//! These operations are conservative — they remove formatting noise
//! that accumulated during a long agent loop (markdown artifacts, the
//! model "thinking out loud" with `...`, accidental double-spacing from
//! tool output concatenation) without touching semantic content.
//!
//! # Scope
//!
//! Applies only to **assistant `Text`** parts older than
//! [`KEEP_LAST_MESSAGES`]. Never touches:
//!
//! * User messages (user intent is sacred).
//! * Tool results (`dedup` and `snippet` already handle those).
//! * Tool calls (JSON arguments must round-trip).
//! * The most recent `KEEP_LAST_MESSAGES` messages (active reasoning).
//! * Anything inside a fenced code block (``` ``` ```).
//!
//! # Always-on
//!
//! No config flag. The operation is semantic-preserving for the
//! assistant's own natural-language output, so the cost of running it
//! when it has nothing to do is trivially small.

use super::ExperimentalStats;

use codetether_provider_core::{ContentPart, Message, Role};

/// Assistant messages more recent than this many-from-end are never pruned.
pub const KEEP_LAST_MESSAGES: usize = 6;
mod prune_low_entropy;
pub use prune_low_entropy::prune_low_entropy;
mod prune_text;
use prune_text::prune_text;
mod collapse_spaces;
use collapse_spaces::collapse_spaces;
#[cfg(test)]
mod tests;
