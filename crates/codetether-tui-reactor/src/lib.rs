//! Terminal-first event selection for interactive applications.
//!
//! Construct [Timers], borrow event inputs through [Sources], and await
//! [select]. The returned [Ready] owns its event so application handlers run
//! after the input borrows have ended.
mod io;
mod lifecycle_binding;
mod ready;
mod select;
mod selection;
mod sources;
mod timers;
pub use io::LoopIo;
pub use ready::Ready;
pub use select::select;
pub use selection::Selection;
pub use sources::Sources;
pub use timers::Timers;

/// Event-handler ordering and post-event draining.
pub mod dispatch;

/// Draw-event loop and resource shutdown ordering.
pub mod driver;
