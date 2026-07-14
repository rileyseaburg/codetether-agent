//! Default-model selection for agent execution.

use crate::agent::Agent;

/// Select the configured model or its provider-specific fallback.
pub(super) fn default_for(agent: &Agent) -> String {
    agent
        .info
        .model
        .clone()
        .unwrap_or_else(|| match agent.provider.name() {
            "zhipuai" | "zai" => "glm-5".to_string(),
            "openrouter" => "z-ai/glm-5".to_string(),
            _ => "glm-5".to_string(),
        })
}
