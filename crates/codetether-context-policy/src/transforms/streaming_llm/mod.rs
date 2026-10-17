//! streaming llm transform rules.
//! StreamingLLM-style middle-drop with attention sinks.
//!
//! Based on *Efficient Streaming Language Models with Attention Sinks*
//! (Xiao et al., 2023; refined 2024). The empirical finding: the first
//! few tokens of a sequence act as "attention sinks" that stabilize
//! later attention distributions, and the *recent* window carries most
//! of the locally-relevant signal. The middle can be dropped with
//! minimal quality loss at long contexts.
//!
//! We apply the same idea at **message granularity** rather than token
//! granularity, which is the right level for an agentic loop:
//!
//! * **Sinks** — the first [`SINK_MESSAGES`] messages (typically the
//!   user's original intent + any system/scaffolding prefix).
//! * **Recent window** — the last [`RECENT_MESSAGES`] messages (active
//!   reasoning the model is building on).
//! * **Middle** — compacted into a single user-visible marker message.
//!
//! # Safety
//!
//! Only activates when history is long enough that the middle is
//! genuinely stale (see [`STREAM_MIN_MESSAGES`]). Never drops content
//! while a tool call is open: we always cut on a boundary where the
//! most recent dropped message is not a `Role::Assistant` with an
//! unresolved tool call. This preserves the tool-call ⇄ tool-result
//! pairing required by every provider's chat API.
//!
//! # Rollout
//!
//! This strategy is intentionally kept separate from the default-safe
//! [`super::apply_all`] path. It is materially lossier than the other
//! transforms because it can remove semantically relevant turns even
//! when the request is still well within the model's context budget.

use super::ExperimentalStats;

use codetether_provider_core::{ContentPart, Message, Role};

/// Number of leading messages preserved as attention sinks.
pub const SINK_MESSAGES: usize = 2;

/// Number of trailing messages preserved as the recency window.
pub const RECENT_MESSAGES: usize = 16;

/// Below this total message count, streaming trim is a no-op.
pub const STREAM_MIN_MESSAGES: usize = SINK_MESSAGES + RECENT_MESSAGES + 4;
mod trim_middle;
pub use trim_middle::trim_middle;
mod has_open_tool_call;
use has_open_tool_call::has_open_tool_call;
mod approx_bytes;
use approx_bytes::approx_bytes;
#[cfg(test)]
mod tests;
