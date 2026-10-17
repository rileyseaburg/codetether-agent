//! Native local/remote routing and message startup bindings.
use super::ApplicationHost;
use crate::{
    session::{Session, SessionEvent},
    tool::{
        ToolResult,
        agent::{message, store},
    },
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_input::Params;
use codetether_child_turn::{MessageHost, Started};
use codetether_collaboration_input::MessageImage;
#[async_trait]
impl MessageHost for ApplicationHost {
    fn resolve(target: &str, owner: Option<&str>) -> Option<(String, String)> {
        store::scope::resolve_for_parent(target, owner).map(|(id, entry)| (id, entry.name))
    }
    async fn remote(target: &str, text: &str, params: &Params) -> Result<ToolResult> {
        message::remote_fallback::execute(target, text, params).await
    }
    async fn begin_message(
        agent_id: &str,
        text: String,
        images: Vec<MessageImage>,
        params: &Params,
    ) -> Result<Started<SessionEvent, Session>> {
        message::start::begin(agent_id, text, images, params).await
    }
}
