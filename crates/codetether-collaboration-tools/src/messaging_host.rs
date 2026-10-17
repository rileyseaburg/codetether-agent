//! Live residency, delivery and listing services supplied by the application.
use crate::{EnsureOpen, Host, Route, RuntimeContext};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Host services needed by messaging tools in addition to lifecycle execution.
///
/// # Implementors
/// Use the existing application residency, communication and live-tree owners.
#[async_trait]
pub trait MessagingHost: Host {
    /// Open the requested child with current parent runtime settings.
    /// # Errors
    /// Returns the existing residency error.
    async fn open(
        target: &str,
        owner: Option<&str>,
        context: &RuntimeContext,
    ) -> Result<EnsureOpen>;
    /// Steer an active child or report why immediate delivery is unavailable.
    async fn steer(target: &str, owner: Option<&str>, message: &str) -> Route;
    /// Queue a message without starting an idle child turn.
    /// # Errors
    /// Returns the existing communication error.
    async fn queue_only(target: &str, owner: Option<&str>, message: String) -> Result<ToolResult>;
    /// Project the current root tree using the existing prefix resolution.
    /// # Errors
    /// Returns the existing tree lookup error.
    fn list(current: &str, path_prefix: Option<&str>) -> Result<Value>;
}
