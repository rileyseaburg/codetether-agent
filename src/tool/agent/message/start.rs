//! Native event specialization for shared message startup.
use crate::session::{Session, SessionEvent};
use codetether_agent_input::Params;
use codetether_child_turn::Started;
use codetether_collaboration_input::MessageImage;
pub(in crate::tool::agent) async fn begin(
    agent_id: &str,
    message: String,
    images: Vec<MessageImage>,
    params: &Params,
) -> anyhow::Result<Started<SessionEvent, Session>> {
    codetether_child_turn::begin_message::<crate::tool::agent::turn_host::ApplicationHost>(
        agent_id, message, images, params,
    )
    .await
}
