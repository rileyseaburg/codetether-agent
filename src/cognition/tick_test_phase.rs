//! Bind shared Test-phase tool dispatch to the native registry and thinker.
use super::{PersonaRuntimeState, ThinkerClient, ThoughtEvent, ThoughtWorkItem};
use crate::tool::ToolRegistry;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
pub(super) async fn run_tools(
    personas: &Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    thinker: Option<&ThinkerClient>,
    tools: &Arc<ToolRegistry>,
    work: &ThoughtWorkItem,
    thinking: &str,
) -> Vec<ThoughtEvent> {
    codetether_cognition_core::execution::dispatch::run_tools(
        personas, thinker, tools, work, thinking,
    )
    .await
}
