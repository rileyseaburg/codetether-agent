//! Approval-aware MCP manager connection helper.

use crate::{Client, McpToolManager};
use anyhow::Result;

impl<C: Client> McpToolManager<C> {
    /// Connect through the host's existing authorization and initialization flow.
    ///
    /// # Arguments
    /// * `command` - Executable to spawn.
    /// * `args` - Arguments passed to that executable.
    /// * `approval_id` - Optional approval receipt for the host policy.
    /// # Returns
    /// A manager sharing the authorized client.
    /// # Errors
    /// Returns host connection, initialization or authorization errors.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example<C:codetether_mcp_tools::Client>()->anyhow::Result<()> {
    /// let _manager=codetether_mcp_tools::McpToolManager::<C>::connect_subprocess_with_approval("server",&[],None).await?;
    /// # Ok(()) }
    /// ```
    pub async fn connect_subprocess_with_approval(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> Result<Self> {
        let client = C::connect_subprocess_with_approval(command, args, approval_id).await?;
        Ok(Self { client })
    }
}
