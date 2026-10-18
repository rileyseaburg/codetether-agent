//! Load effective invocation configuration before native and shared checks.
use super::Host;
use codetether_config::Config;
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::Path;
/// Load argument-selected configuration and evaluate an invocation.
/// # Arguments
/// Receives the tool name and original arguments.
/// # Returns
/// A blocking result or None, using default config after a load failure.
/// # Examples
/// ```no_run
/// async fn check<H: codetether_runtime_policy::evaluation::Host>() {
///     let _ = codetether_runtime_policy::evaluation::evaluate::<H>("read", &serde_json::json!({})).await;
/// }
/// ```
pub async fn evaluate<H: Host>(tool_name: &str, args: &Value) -> Option<ToolResult> {
    let config = match crate::workspace::from_args(args) {
        Some(path) => Config::load_for_workspace(path).await,
        None => Config::load().await,
    }
    .unwrap_or_default();
    super::with_config::<H>(&config, tool_name, args)
}

/// Evaluate an invocation using an explicit workspace's configuration.
/// # Arguments
/// Receives tool name, original arguments and workspace path.
/// # Returns
/// A blocking result or None, using default config after a load failure.
/// # Examples
/// ```no_run
/// async fn check<H: codetether_runtime_policy::evaluation::Host>() {
///     let _ = codetether_runtime_policy::evaluation::for_workspace::<H>("read", &serde_json::json!({}), std::path::Path::new(".")).await;
/// }
/// ```
pub async fn for_workspace<H: Host>(
    tool_name: &str,
    args: &Value,
    workspace: &Path,
) -> Option<ToolResult> {
    let config = Config::load_for_workspace(workspace)
        .await
        .unwrap_or_default();
    super::with_config::<H>(&config, tool_name, args)
}
