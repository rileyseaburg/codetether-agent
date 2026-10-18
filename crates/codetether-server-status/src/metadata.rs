//! Compatibility provider names and built-in agent metadata.
use axum::Json;

pub(super) async fn list_providers() -> Json<Vec<String>> {
    Json(vec![
        "openai".to_string(),
        "anthropic".to_string(),
        "google".to_string(),
    ])
}

pub(super) async fn list_agents() -> Json<Vec<codetether_agent_core::AgentInfo>> {
    let registry = codetether_agent_core::AgentRegistry::with_builtins();
    Json(registry.list().into_iter().cloned().collect())
}
