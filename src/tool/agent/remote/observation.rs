//! Parent-scoped runtime evidence for remote A2A agent turns.

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
pub(in crate::tool::agent) mod types;
#[path = "observation/update.rs"]
mod update;

pub(in crate::tool::agent) use activity::record as record_activity;
pub(in crate::tool::agent) use query::{live_trace, snapshots, transcript};
pub(in crate::tool::agent) use update::begin;

#[cfg(test)]
#[path = "observation_activity_tests.rs"]
mod activity_tests;
#[cfg(test)]
#[path = "observation_tests.rs"]
mod tests;
