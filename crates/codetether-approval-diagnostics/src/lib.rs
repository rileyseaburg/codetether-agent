//! Proposed-file language-server analysis for interactive approvals.
//!
//! arguments recovers current or legacy request inputs. inspect reconstructs
//! proposed content and asks the caller's managed clients for diagnostics.
//! Manager ownership, task scheduling and queue publication belong to callers.
mod arguments;
mod inspect;
mod report;
pub use arguments::arguments;
pub use inspect::inspect;
