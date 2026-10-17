//! thinking prune transform rules.
//! Strip extended-thinking blocks from older messages.
//!
//! Modern reasoning models (Claude extended thinking, DeepSeek R1,
//! GPT-5 reasoning, Gemini thought summaries) emit `Thinking` content
//! parts that can be **10-100×** larger than the assistant's actual
//! reply. These blocks help the *current* turn's decision but carry
//! almost no value once the turn has produced its tool calls and the
//! loop has moved on — the final answer/action already reflects them.
//!
//! This module removes [`ContentPart::Thinking`] from every message
//! older than [`KEEP_LAST_MESSAGES`]. Recent thinking is preserved so
//! the model can still reference its own recent chain-of-thought.
//!
//! # Safety
//!
//! * Providers that inject thinking for correctness (cache-coherent
//!   thought signatures on Gemini `ToolCall`) are unaffected — those
//!   signatures live on `ContentPart::ToolCall::thought_signature`,
//!   not on `Thinking` blocks.
//! * An assistant message whose *only* content was a thinking block
//!   becomes empty; such messages are removed entirely to keep the
//!   buffer a valid provider-consumable shape.
//!
//! # Always-on
//!
//! No config. Thinking blocks are known to be non-essential after the
//! turn completes; stripping them is the single highest-ROI shrink for
//! reasoning-heavy agent loops.

use super::ExperimentalStats;

use codetether_provider_core::{ContentPart, Message, Role};

/// Keep thinking blocks in this many trailing messages.
pub const KEEP_LAST_MESSAGES: usize = 4;
mod prune_thinking;
pub use prune_thinking::prune_thinking;
mod thinking_bytes;
use thinking_bytes::thinking_bytes;
#[cfg(test)]
mod tests;
