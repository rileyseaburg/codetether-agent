//! Adaptive request-budget enforcement over provider message buffers.
//!
//! [enforce_on_messages] runs keep-last compaction and terminal fallback,
//! emitting shared session lifecycle events. [terminal_truncate_messages]
//! exposes the existing deterministic last-resort operation.
mod enforce;
mod enforce_attempt;
mod enforce_budget;
mod enforce_cascade;
mod enforce_events;
mod enforce_run;
mod enforce_run_tail;
mod enforce_terminal;
mod enforce_terminal_events;
mod enforce_terminal_overrun;
mod terminal;
mod terminal_drop;
pub use enforce::enforce_on_messages;
pub use terminal::terminal_truncate_messages;
#[cfg(test)]
mod tests;

mod cascade_finish;
mod env_from_check;
mod event_completed;
mod event_send;
mod event_started;
mod start;
