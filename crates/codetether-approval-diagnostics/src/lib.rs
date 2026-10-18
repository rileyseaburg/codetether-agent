//! Proposed-file language-server analysis for interactive approvals.
//!
//! arguments recovers current or legacy request inputs. inspect reconstructs
//! proposed content and asks the caller's managed clients for diagnostics.
//! start schedules inspection and queue publication using the caller's cached manager.
mod arguments;
mod inspect;
mod report;
pub use arguments::arguments;
pub use inspect::inspect;

mod start;
pub use start::start;
