//! Background message/stream admission and execution adapters.
use super::*;
mod event_data;
use codetether_a2a_http::task_state::stream::events;
mod run;
pub(super) mod spawn;
pub(super) use codetether_a2a_http::admission::stream as handle_message_stream;
mod registry;

mod host;
