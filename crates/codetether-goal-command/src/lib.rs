//! Human-facing goal command routing and persistence.
//!
//! [run] parses the text following a goal command. [Host] preserves the
//! application's journal paths, runtime settings and authenticated user edits.
mod auto;
mod budget;
mod dispatch;
mod host;
mod show;
mod status;
mod write;
pub use dispatch::run;
pub use host::Host;

/// Session-independent human goal/task input and deferred continuation.
pub mod input;

/// Frontend result presentation for authorized human goal commands.
pub mod presentation;
