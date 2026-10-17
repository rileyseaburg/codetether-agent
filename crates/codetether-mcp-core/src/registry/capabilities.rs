//! Generic MCP registry capabilities operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Check if a server has a specific capability
    pub async fn has_capability(&self, name: &str, capability: &str) -> bool {
        let caps = self.server_capabilities.read().await;
        caps.get(name)
            .map(|c| match capability {
                "tools" => c.tools.is_some(),
                "resources" => c.resources.is_some(),
                "prompts" => c.prompts.is_some(),
                "logging" => c.logging.is_some(),
                _ => false,
            })
            .unwrap_or(false)
    }

    /// List servers that have a specific capability
    pub async fn list_by_capability(&self, capability: &str) -> Vec<String> {
        let mut result = Vec::new();
        let caps = self.server_capabilities.read().await;

        for (name, caps) in caps.iter() {
            let has_cap = match capability {
                "tools" => caps.tools.is_some(),
                "resources" => caps.resources.is_some(),
                "prompts" => caps.prompts.is_some(),
                "logging" => caps.logging.is_some(),
                _ => false,
            };
            if has_cap {
                result.push(name.clone());
            }
        }

        result
    }
}
