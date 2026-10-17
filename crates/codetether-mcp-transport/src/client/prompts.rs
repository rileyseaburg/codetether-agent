//! Read remote prompt metadata and templates.
use super::*;
impl RemoteClient {
    /// List remote prompt templates.
    ///
    /// # Arguments
    /// Borrows the client.
    ///
    /// # Returns
    /// The remote prompt descriptions.
    ///
    /// # Errors
    /// Transport, wire or decoding errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.list_prompts().await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>> {
        self.connection.list_prompts().await
    }

    /// Retrieve a prompt template with arguments.
    ///
    /// # Arguments
    /// name identifies the prompt; arguments supplies its inputs.
    ///
    /// # Returns
    /// The rendered remote prompt.
    ///
    /// # Errors
    /// Transport, wire or decoding errors.
    ///
    /// # Examples
    /// ```rust,no_run
    /// async fn example(client: &codetether_mcp_transport::client::RemoteClient) -> anyhow::Result<()> {
    /// let _ = client.get_prompt("example", serde_json::json!({})).await?;
    /// Ok(())
    /// }
    /// ```
    pub async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult> {
        self.connection.get_prompt(name, arguments).await
    }
}
