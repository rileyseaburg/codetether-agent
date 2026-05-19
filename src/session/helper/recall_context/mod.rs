//! Budget-aware message flattening for session recall.
//!
//! Prevents OOM by enforcing a token budget when flattening sessions
//! for the `session_recall` tool. Skips thinking blocks, truncates
//! large tool results, and stops once the budget is exhausted.

pub mod flatten;
pub mod render;

pub use flatten::flatten_messages as messages_to_recall_context;

#[cfg(test)]
mod tests;
