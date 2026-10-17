//! Public prompt-handler catalog access.
use super::*;
impl McpServer {
    /// Register a prompt handler
    pub async fn register_prompt(&self, name: &str, handler: McpPromptHandler) {
        self.catalog.register_prompt(name, handler).await
    }

    /// Get a prompt handler by name
    pub async fn get_prompt_handler(&self, name: &str) -> Option<McpPromptHandler> {
        self.catalog.get_prompt_handler(name).await
    }

    /// List all registered prompt names
    pub async fn list_prompts(&self) -> Vec<String> {
        self.catalog.list_prompts().await
    }
}
