//! Bind shared invocation to the native catalog and optional event bus.
use super::{Application, ApplicationServer};
use codetether_mcp_core::types::JsonRpcError;
use serde_json::Value;
impl<H: Application> ApplicationServer<H> {
    pub(super) async fn handle_call_tool(
        &self,
        params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        crate::invoke(&self.inner.catalog, self.inner.agent_bus.as_deref(), params).await
    }
}
