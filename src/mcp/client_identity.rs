//! Stable identity used for registry tracking and approval scope.

use super::McpClient;

impl McpClient {
    /// Get the server name if set.
    pub async fn server_name(&self) -> Option<String> {
        self.server_name.read().await.clone()
    }
    /// Set the server name for registry tracking.
    pub async fn set_server_name(&self, name: String) {
        *self.server_name.write().await = Some(name);
    }

    pub(super) async fn approval_identity(&self) -> String {
        self.server_name()
            .await
            .unwrap_or_else(|| self.policy_identity.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::McpClient;
    use crate::mcp::NullTransport;
    use std::sync::Arc;

    #[tokio::test]
    async fn same_tool_on_different_servers_has_distinct_approval_scope() {
        let first = McpClient::new(Arc::new(NullTransport::new()));
        let second = McpClient::new(Arc::new(NullTransport::new()));
        first.set_server_name("server-a".into()).await;
        second.set_server_name("server-b".into()).await;
        let first = format!("mcp:{}:deploy", first.approval_identity().await);
        let second = format!("mcp:{}:deploy", second.approval_identity().await);
        assert_ne!(first, second);
        let args = serde_json::json!({"target": "prod"});
        assert_ne!(
            crate::runtime_policy::invocation_scope::for_tool(&first, &args).resource,
            crate::runtime_policy::invocation_scope::for_tool(&second, &args).resource,
        );
    }
}
