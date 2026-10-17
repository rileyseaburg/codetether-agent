//! Worker registration agents projection.

pub fn builtin_agent_defs() -> Vec<serde_json::Value> {
    codetether_agent_core::AgentRegistry::with_builtins()
        .list()
        .iter()
        .map(|info| {
            serde_json::json!({
                "name": info.name, "description": info.description, "mode": format!("{:?}", info.mode).to_lowercase(),
                "native": info.native, "hidden": info.hidden, "model": info.model, "temperature": info.temperature,
                "top_p": info.top_p, "max_steps": info.max_steps,
            })
        })
        .collect()
}
