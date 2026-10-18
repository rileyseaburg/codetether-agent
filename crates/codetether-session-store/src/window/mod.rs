//! Bound a loaded working set without deleting durable session history.
//!
//! [WorkingSet] borrows independent native fields so [before_append] can keep
//! the checkpoint locked while updating vectors and invalidating summaries.
mod evict;
mod view;
pub use evict::before_append;
pub use view::WorkingSet;
/// Maximum messages retained by interactive and background readers.
pub const WINDOW: usize = 1_000;
