//! # Thread Event Mapping
//!
//! Converts live session, tool, and patch lifecycle signals into durable
//! [`ThreadEvent`](crate::session::thread_store::ThreadEvent) records.

mod approval;
mod command;
mod context;
mod ids;
mod item;
mod lifecycle;
mod mapper;
mod patch;
mod patch_metadata;
mod session;
mod time;
mod tool;
mod tool_close;
mod tool_events;
mod tool_item;
mod tool_metadata;

pub use context::ThreadEventContext;
pub use ids::{thread_id_for_session, turn_id_for_session};
pub use mapper::ThreadEventMapper;
pub use time::timestamp_ms;

#[cfg(test)]
mod tests;
