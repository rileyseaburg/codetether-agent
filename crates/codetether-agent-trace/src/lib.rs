//! Live child-agent transcript snapshots and event reduction.
//!
//! [begin] creates the current turn snapshot. Borrow native event fields into
//! [TraceEvent] and pass them to [observe]; [snapshot] clones the current view
//! for display and [clear] removes it once durable history is available.
mod entry;
mod event;
mod record;
mod session_event;
pub use session_event::{observe_session, project};
mod snapshot;
mod store;
mod tool;
pub use entry::LiveTraceEntry;
pub use event::TraceEvent;
pub use snapshot::LiveTraceSnapshot;
pub use store::{begin, clear, observe, snapshot};
