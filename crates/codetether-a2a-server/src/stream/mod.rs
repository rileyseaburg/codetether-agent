//! Background message/stream admission and execution adapters.
use super::*;
mod event_data;
use codetether_a2a_http::task_state::stream::events;
mod run;
pub(super) mod spawn;
pub(super) use codetether_a2a_http::admission::stream as handle_message_stream;

mod host;

async fn load_registry<H: Application>(
    tasks: &DashMap<String, Task>,
    task_id: &str,
) -> Option<H::Registry> {
    codetether_a2a_http::task_state::stream::registry::load(tasks, task_id, H::registry()).await
}
