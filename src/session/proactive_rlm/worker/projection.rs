//! Native materialized-view lookup for shared proactive orchestration.
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
#[async_trait]
impl codetether_proactive_context::Host for super::host::NativeHost {
    async fn projection(&self, id: &str, name: &str) -> Result<Option<Value>> {
        crate::session::store::projection::read(id, name).await
    }
}
