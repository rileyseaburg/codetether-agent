//! Snapshot the current conversation into a named background agent.
//!
//! [handle] checks activity, persists the source, creates a bounded snapshot
//! and registers it. Implement [Host] for native agent storage and UI updates.
mod command;
mod host;
mod name;
mod register;
pub use command::handle;
pub use host::Host;
