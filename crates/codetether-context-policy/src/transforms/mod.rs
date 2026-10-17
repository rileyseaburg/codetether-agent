//! Shared context transforms and their aggregate statistics.
//!
//! [pairing] restores tool-call/result consistency; [ExperimentalStats] tracks
//! replacements and byte savings for callers composing multiple transforms.
mod stats;
pub use stats::ExperimentalStats;
/// Orphan call/result repair independent of session state.
pub mod pairing;

/// dedup message transformation.
pub mod dedup;
/// lingua message transformation.
pub mod lingua;
/// snippet message transformation.
pub mod snippet;
/// streaming llm message transformation.
pub mod streaming_llm;
/// thinking prune message transformation.
pub mod thinking_prune;
/// tool call dedup message transformation.
pub mod tool_call_dedup;

/// Default transform ordering with an injected reasoning-repair hook.
pub mod pipeline;
/// Message/JSON conversion for reasoning repair.
pub mod reasoning;
