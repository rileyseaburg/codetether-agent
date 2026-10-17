//! Background message/stream admission and execution adapters.
use super::*;
use codetether_a2a_http::task_state::stream::complete;
mod entry;
mod event_data;
mod events;
use codetether_a2a_http::task_state::stream::failure;
use codetether_a2a_http::task_state::stream::open;
mod run;
mod spawn;
use codetether_a2a_http::task_state::stream::start_failure;
use codetether_a2a_http::task_state::stream::text;
pub(super) use entry::handle_message_stream;
mod registry;
