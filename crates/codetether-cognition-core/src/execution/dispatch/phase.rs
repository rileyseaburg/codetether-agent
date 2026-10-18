use super::Registry;
use crate::{
    PersonaRuntimeState, ThoughtEvent, ThoughtWorkItem,
    persona::progress::{allowed_tools, mark_progress},
    thinker::client::{Client, Host},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
/// Run any tool requests the thought asked for, within the persona's leases.
///
/// Returns the resulting tool events; progress is marked for each one.
pub async fn run_tools<H: Host, R: Registry + ?Sized>(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    thinker: Option<&Client<H>>,
    tools: &Arc<R>,
    work: &ThoughtWorkItem,
    thinking: &str,
) -> Vec<ThoughtEvent> {
    let allowed = allowed_tools(personas, &work.persona_id).await;
    if allowed.is_empty() {
        return Vec::new();
    }
    let results =
        super::extract::execute_tool_requests(thinker, tools, &work.persona_id, thinking, &allowed)
            .await;
    for _ in &results {
        mark_progress(personas, &work.persona_id).await;
    }
    results
}
