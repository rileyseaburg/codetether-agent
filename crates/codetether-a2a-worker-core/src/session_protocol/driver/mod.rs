//! Worker session-loop sequencing over host-owned execution and persistence.
//!
//! [run] controls step budgets and continuation. [SessionLoopHost] supplies
//! application state operations and reports [Continuation] after each message.
mod host;
mod run;
pub use host::{Continuation, SessionLoopHost};
pub use run::run;

mod scoped;
pub use scoped::{ScopedHost, run_scoped};
