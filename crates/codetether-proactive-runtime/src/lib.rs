//! Process-wide proactive summary runtimes and latest-generation scheduling.
//!
//! [register] binds provider settings to a session. [schedule] coalesces new
//! work and launches a worker callback only when needed. [acquire] bounds
//! concurrent proactive jobs with the existing process-wide semaphore.
mod capacity;
mod registry;
mod registry_status;
mod schedule;
mod types;
pub use capacity::acquire;
pub use registry::{enqueue, register, settle, take};
pub use registry_status::{exists, is_current, remove};
pub use schedule::schedule;
pub use types::{Runtime, Snapshot};
