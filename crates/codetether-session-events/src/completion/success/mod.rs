//! Successful completion ownership, persistence and presentation sequencing.
//!
//! [apply] rejects stale results before restoring the current session. [Host]
//! binds runtime state; [ActiveSession] retains the restored session borrow
//! through bus attachment, recovery reset, persistence and list refresh.
mod active;
mod host;
mod run;
pub use active::ActiveSession;
pub use host::Host;
pub use run::apply;
