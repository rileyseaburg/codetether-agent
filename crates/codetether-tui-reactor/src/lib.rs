//! Terminal-first event selection for interactive applications.
//!
//! Construct [Timers], borrow event inputs through [Sources], and await
//! [select]. The returned [Ready] owns its event so application handlers run
//! after the input borrows have ended.
mod ready;
mod select;
mod sources;
mod timers;
pub use ready::Ready;
pub use select::select;
pub use sources::Sources;
pub use timers::Timers;
