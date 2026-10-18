//! Native tool execution during the Test phase.
use super::{
    PersonaRuntimeState, ThinkerClient, ThoughtEvent, ThoughtWorkItem, executor,
    tick_progress::{allowed_tools, mark_progress},
};
use crate::tool::ToolRegistry;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
/// Run any tool requests the thought asked for, within the persona's leases.
///
/// Returns the resulting tool events; progress is marked for each one.
pub(super) async fn run_tools(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    thinker: Option<&ThinkerClient>,
    tools: &Arc<ToolRegistry>,
    work: &ThoughtWorkItem,
    thinking: &str,
) -> Vec<ThoughtEvent> {
    let allowed = allowed_tools(personas, &work.persona_id).await;
    if allowed.is_empty() {
        return Vec::new();
    }
    let results =
        executor::execute_tool_requests(thinker, tools, &work.persona_id, thinking, &allowed).await;
    for _ in &results {
        mark_progress(personas, &work.persona_id).await;
    }
    results
}
