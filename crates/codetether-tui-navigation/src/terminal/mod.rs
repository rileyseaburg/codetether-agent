//! Terminal event routing with goal-editor paste priority.
//!
//! Implement [Host] for native handlers and call [dispatch] with the next
//! terminal-stream item. Stream errors, closure and other events are ignored.
mod binding;
mod host;
mod run;
pub use host::Host;
pub use run::dispatch;
