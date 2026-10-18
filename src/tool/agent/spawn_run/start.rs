//! Native event specialization for shared first-turn startup.
use crate::session::{Session, SessionEvent};
use codetether_child_turn::Started;
pub(in crate::tool::agent) async fn begin(
    agent_id: &str,
) -> anyhow::Result<Started<SessionEvent, Session>> {
    codetether_child_turn::begin_first::<crate::tool::agent::turn_host::ApplicationHost>(agent_id)
        .await
}
