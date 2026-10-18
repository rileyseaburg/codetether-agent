//! Background message/stream admission and execution adapters.
use super::*;
mod event_data;
use codetether_a2a_http::task_state::stream::events;
mod run;
pub(super) mod spawn;
pub(super) use codetether_a2a_http::admission::stream as handle_message_stream;

mod host;

async fn load_registry(
    tasks: &DashMap<String, Task>,
    task_id: &str,
) -> Option<Arc<crate::provider::ProviderRegistry>> {
    codetether_a2a_http::task_state::stream::registry::load(
        tasks,
        task_id,
        crate::provider::ProviderRegistry::shared_from_vault(),
    )
    .await
}
