//! Parent-scoped runtime evidence for remote A2A agent turns.
//!
//! [begin] starts a guarded turn; [snapshots], [transcript] and [live_trace]
//! expose its evidence. Settling or dropping the guard records completion.

#[path = "observation/activity.rs"]
mod activity;
#[path = "observation/finish.rs"]
mod finish;
#[path = "observation/guard.rs"]
mod guard;
#[path = "observation/message.rs"]
mod message;
#[path = "observation/projection.rs"]
mod projection;
#[path = "observation/projection_value.rs"]
mod projection_value;
#[path = "observation/query.rs"]
mod query;
#[path = "observation/store.rs"]
mod store;
#[path = "observation/types.rs"]
pub mod types;
#[path = "observation/update.rs"]
mod update;

pub use activity::record as record_activity;
pub use query::{live_trace, snapshots, transcript};
pub use update::begin;

#[cfg(test)]
#[path = "observation_activity_tests.rs"]
mod activity_tests;
#[cfg(test)]
#[path = "observation_tests.rs"]
mod tests;

#[path = "observation/projection_apply.rs"]
mod projection_apply;
