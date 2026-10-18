//! Application MCP client operations adapter.
use super::*;
impl<H: Application> McpClient<H> {
    /// Call a tool
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        self.call_tool_checked(name, arguments).await
    }

    /// List remote resources.
    pub async fn list_resources(&self) -> Result<Vec<McpResource>> {
        self.inner.list_resources().await
    }

    /// Read one remote resource.
    pub async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        self.inner.read_resource(uri).await
    }

    /// List remote prompt templates.
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>> {
        self.inner.list_prompts().await
    }

    /// Retrieve a prompt template with arguments.
    pub async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult> {
        self.inner.get_prompt(name, arguments).await
    }

    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        self.inner.close().await
    }
}
