//! Forward collaboration operations through the existing application runtime.
use super::{context::RuntimeContext, host::ApplicationHost};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Map, Value};
pub(super) async fn execute(
    context: &RuntimeContext,
    payload: Map<String, Value>,
) -> Result<ToolResult> {
    codetether_collaboration_tools::execute::<ApplicationHost>(context, payload).await
}
