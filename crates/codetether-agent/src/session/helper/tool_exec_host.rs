//! Connect shared tool execution to native registry and authorization gates.
use crate::tool::{Tool, ToolRegistry};
use async_trait::async_trait;
use codetether_tool_reporting::ToolTuple;
use serde_json::Value;
use std::sync::Arc;
pub(in crate::session::helper) struct NativeHost<'a> {
    pub registry: &'a ToolRegistry,
}
#[async_trait]
impl codetether_tool_execution::Host for NativeHost<'_> {
    fn resolve(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.registry.get(name)
    }
    async fn policy(&self, name: &str, args: &Value) -> Option<ToolTuple> {
        crate::session::helper::tool_policy::blocked(name, args).await
    }
    async fn coordinate(&self, name: &str, args: &Value) -> Option<ToolTuple> {
        crate::session::helper::workspace_coordination::blocked(name, args).await
    }
}
