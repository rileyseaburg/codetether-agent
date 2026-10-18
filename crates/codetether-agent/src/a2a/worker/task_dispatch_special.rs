//! Application forage hook for native special-task dispatch.
use anyhow::Result;
use serde_json::{Map, Value};
pub(super) struct Forage;
impl codetether_native_worker::special::Forage for Forage {
    async fn execute(
        title: &str,
        prompt: &str,
        metadata: &Map<String, Value>,
        selected_model: Option<String>,
    ) -> Result<String> {
        codetether_forage_runtime::worker_task::handle::<crate::forage::execution_host::Native>(
            title,
            prompt,
            metadata,
            selected_model,
        )
        .await
    }
}
