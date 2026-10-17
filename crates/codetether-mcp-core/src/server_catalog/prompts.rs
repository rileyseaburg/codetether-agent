//! MCP server catalog prompts operations.
use super::*;
impl ServerCatalog {
    /// Register a prompt handler
    pub async fn register_prompt(&self, name: &str, handler: McpPromptHandler) {
        let mut prompts = self.prompts.write().await;
        prompts.insert(name.to_string(), handler);
        tracing::debug!(target: "codetether_agent::mcp::server", "Registered MCP prompt: {}", name);
    }

    /// Get a prompt handler by name
    pub async fn get_prompt_handler(&self, name: &str) -> Option<McpPromptHandler> {
        let prompts = self.prompts.read().await;
        prompts.get(name).cloned()
    }

    /// List all registered prompt names
    pub async fn list_prompts(&self) -> Vec<String> {
        let prompts = self.prompts.read().await;
        prompts.keys().cloned().collect()
    }
}
