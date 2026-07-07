//! Session history compression via the RLM router.
//!
//! Keeps the prompt under the model's token budget. Invoked automatically
//! at the start of every agent step by [`Session::run_loop`](crate::session::Session).
//!
//! ## Strategy
//!
//! 1. Estimate the current request token cost (system + messages + tools).
//! 2. If it exceeds 90% of the model's usable budget, compress the prefix
//!    via a ready background RLM summary or deterministic chunker pass,
//!    keeping the most recent `keep_last` messages verbatim.
//! 3. Progressively shrink `keep_last` (16 → 12 → 8 → 6 → 3 → 1) until the
//!    budget is met or nothing more can be compressed.
//!
//! The compressed prefix is replaced by a single synthetic assistant
//! message tagged `[AUTO CONTEXT COMPRESSION]` so the model sees a
//! coherent summary rather than a truncated tail.
//!
//! Terminal truncation drops the oldest messages outright (no summary)
//! and is deliberately a distinct event from `CompactionCompleted` so
//! consumers can warn the user about silent context loss.
//!
//! Submodule layout (SRP split, one concern per file):
//! * [`context`] — the [`CompressContext`] snapshot type.
//! * [`keep_last`] / [`keep_last_session`] — keep-last prefix compression.
//! * [`oversized`] / [`oversized_text`] — oversized-last-message handling.
//! * [`enforce`] + `enforce_*` — adaptive budget-cascade enforcement.
//! * [`terminal`] / [`terminal_marker`] — terminal truncation last resort.
//! * [`shrink`] / `shrink_*` — payload head/tail shrinking.

#[path = "../compression_defer.rs"]
mod compression_defer;
#[path = "../compression_last_message.rs"]
mod compression_last_message;
#[path = "../compression_summary.rs"]
mod compression_summary;
mod context;
pub(crate) mod dropped_toc;
#[cfg(test)]
mod dropped_toc_tests;
mod enforce;
mod enforce_attempt;
mod enforce_budget;
mod enforce_cascade;
mod enforce_events;
mod enforce_run;
mod enforce_run_tail;
mod enforce_session;
mod enforce_terminal;
mod enforce_terminal_events;
mod enforce_terminal_overrun;
mod keep_last;
mod keep_last_session;
mod oversized;
mod oversized_text;
mod shrink;
mod shrink_caps;
mod shrink_part;
mod terminal;
mod terminal_marker;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_context;
#[cfg(test)]
mod tests_shrink;

pub(crate) use context::CompressContext;
pub(crate) use enforce::enforce_on_messages;
pub use enforce_session::enforce_context_window;
pub(crate) use keep_last::compress_messages_keep_last;
pub use keep_last_session::compress_history_keep_last;
pub(crate) use oversized::compress_last_message_if_oversized;
