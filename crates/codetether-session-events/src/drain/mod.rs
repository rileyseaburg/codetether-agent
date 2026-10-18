//! Bounded queued-event delivery with cumulative-text coalescing.
//! [run] retains only the latest adjacent text snapshot, flushing it before
//! other events and at the end. A native [Handler] applies delivered events.
mod modes;
pub use modes::{batch, before_notice};
mod handler;
mod run;
pub use handler::Handler;
pub use run::run;
/// Maximum events consumed during an ordinary UI pass.
pub const BATCH_LIMIT: usize = 64;
/// Maximum events consumed before applying a session completion notice.
pub const NOTICE_LIMIT: usize = 1024;
