//! Native effects used by shared child-turn coordination.
pub(super) use super::event_loop::ApplicationHost;
mod finalize;
mod mailbox;
mod message;
mod prompt;
mod runtime;
mod startup;
