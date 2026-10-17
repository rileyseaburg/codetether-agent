//! Background message/stream admission and execution adapters.
use super::*;
mod entry;
mod event_data;
use codetether_a2a_http::task_state::stream::events;
use codetether_a2a_http::task_state::stream::open;
mod run;
mod spawn;
use codetether_a2a_http::task_state::stream::text;
pub(super) use entry::handle_message_stream;
mod registry;

mod host;
