//! Background message/stream admission and execution adapters.
use super::*;
mod artifact;
mod complete;
mod completion_events;
mod entry;
mod event_data;
mod events;
mod failure;
mod open;
mod reply;
mod run;
mod spawn;
mod start_failure;
mod text;
pub(super) use entry::handle_message_stream;
mod registry;
