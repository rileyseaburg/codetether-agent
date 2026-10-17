//! Bind shared messaging tools to the existing application owners.
use super::{context::RuntimeContext, host::ApplicationHost};
use crate::tool::{
    ToolResult,
    agent::{collaboration_runtime::agent_tree, communication, residency},
};
use anyhow::Result;
use codetether_collaboration_tools::{EnsureOpen, Route};
use serde_json::{Value, json};
#[async_trait::async_trait]
impl codetether_collaboration_tools::MessagingHost for ApplicationHost {
    async fn open(
        target: &str,
        owner: Option<&str>,
        context: &RuntimeContext,
    ) -> Result<EnsureOpen> {
        residency::open(target, owner, super::context::resume_config(context)).await
    }
    async fn steer(target: &str, owner: Option<&str>, message: &str) -> Route {
        communication::steer(target, owner, message).await
    }
    async fn queue_only(target: &str, owner: Option<&str>, message: String) -> Result<ToolResult> {
        communication::queue_only(target, owner, message).await
    }
    fn list(current: &str, path_prefix: Option<&str>) -> Result<Value> {
        Ok(json!(agent_tree::list(current, path_prefix)?))
    }
}
