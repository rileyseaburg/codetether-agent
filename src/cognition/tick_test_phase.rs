//! Check-result reporting and tool execution during the Test phase.

use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::tick_progress::{allowed_tools, mark_progress};
use super::{
    PersonaRuntimeState, ThinkerClient, ThoughtEvent, ThoughtEventType, ThoughtResult,
    ThoughtWorkItem, executor, text_util,
};
use crate::tool::ToolRegistry;

/// Emit the check-result event for a completed Test-phase thought.
pub(super) fn check_result_event(work: &ThoughtWorkItem, thought: &ThoughtResult) -> ThoughtEvent {
    ThoughtEvent {
        id: Uuid::new_v4().to_string(),
        event_type: ThoughtEventType::CheckResult,
        persona_id: Some(work.persona_id.clone()),
        swarm_id: work.swarm_id.clone(),
        timestamp: Utc::now(),
        payload: json!({
            "phase": work.phase.as_str(),
            "thought_count": work.thought_count,
            "result_excerpt": text_util::trim_for_storage(&thought.thinking, 280),
            "source": thought.source,
            "model": thought.model,
        }),
    }
}

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
