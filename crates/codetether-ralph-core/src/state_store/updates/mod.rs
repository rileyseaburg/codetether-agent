//! Nonblocking run-store updates with owned snapshots.
//!
//! Each function skips absent stores, captures the existing payload and
//! schedules a background write. Hosts choose the lifecycle points; failures
//! are logged without changing execution outcomes.
use super::{RalphRunState, RalphStateStore, StoryResultEntry};
use crate::{RalphConfig, RalphState, RalphStatus, UserStory};
use std::sync::Arc;
mod checkpoint;
mod completion;
mod initial;
mod iteration;
mod spawn;
mod story;
pub use checkpoint::checkpoint;
pub use completion::completion;
pub use initial::initial;
pub use iteration::iteration;
use spawn::spawn_update;
pub use story::story;
