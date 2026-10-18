//! Swarm provider selection and matching outcome feedback.
//!
//! [choose_provider_for_subtask] ranks available providers using the current
//! delegation state. [record_subtask_outcome] updates that same specialty bucket;
//! [bucket_for_specialty] is the shared projection used by both operations.
mod bucket;
mod outcome;
mod selection;
pub use bucket::bucket_for_specialty;
pub use outcome::record_subtask_outcome;
pub use selection::choose_provider_for_subtask;
