//! Shared readiness handling with application-owned durable residency.
use super::{context::RuntimeContext, host::ApplicationHost};
use crate::tool::ToolResult;
use anyhow::Result;
pub(super) async fn ready(context: &RuntimeContext, target: &str) -> Result<Option<ToolResult>> {
    codetether_collaboration_tools::ensure_ready::<ApplicationHost>(context, target).await
}
