//! MCP action selection with root control-plane and project settings.
use crate::{Host, serve};
use codetether_cli_options::McpArgs;
use std::path::PathBuf;

/// Resolve subprocess arguments and execute the selected MCP action.
/// # Arguments
/// Receives MCP options, the root control-plane URL and token, the project
/// directory and whether that project was explicitly selected.
/// # Returns
/// Unit when the selected client command or stdio server finishes.
/// # Errors
/// Returns command validation, connection, policy or server execution errors.
/// # Examples
/// ```no_run
/// # async fn example<C: codetether_mcp_cli::Host, S: codetether_mcp_cli::serve::Host>(
/// # args: codetether_cli_options::McpArgs) -> anyhow::Result<()> {
/// codetether_mcp_cli::dispatch::<C, S>(args, &None, &None, &None, false).await
/// # }
/// ```
pub async fn dispatch<C: Host, S: serve::Host>(
    args: McpArgs,
    url: &Option<String>,
    token: &Option<String>,
    project: &Option<PathBuf>,
    explicit: bool,
) -> anyhow::Result<()> {
    // Resolve before selecting the action, preserving root CLI behavior.
    let resolved_command = crate::resolve_command(&args);
    match args.action.as_str() {
        "serve" => serve::serve::<S>(&args, url, token, project, explicit).await,
        _ => crate::execute::<C>(args, resolved_command).await,
    }
}
