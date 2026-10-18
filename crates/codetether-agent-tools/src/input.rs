//! Parse and enrich serialized tool input before dispatch.
use crate::*;
/// Prepare a serialized session call and execute it through the same dispatcher.
/// # Arguments
/// Tool host, live session, call ID, tool name and serialized arguments.
/// # Returns
/// A structured parse error or the dispatched tool result.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_tools::*;
/// # async fn example<H: Host>(host: &H, session: &H::Session) {
/// execute_for_session(host, session, "call-1", "tool", "{}").await;
/// # }
/// ```
pub async fn execute_for_session<H: Host>(
    host: &H,
    session: &H::Session,
    call_id: &str,
    name: &str,
    arguments: &str,
) -> ToolResult {
    let input = match prepare::<H>(session, call_id, name, arguments) {
        Ok(value) => value,
        Err(error) => return ToolResult::error(error),
    };
    crate::execute_value(host, name, input).await
}

/// Parse arguments and apply session and image-context enrichment.
/// # Arguments
/// Session, call ID, tool name and serialized arguments.
/// # Returns
/// The enriched JSON input.
/// # Errors
/// Returns the original JSON parse error description.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_tools::*;
/// # async fn example<H: Host>(host: &H, session: &H::Session) {
/// let _ = prepare::<H>(session, "call-1", "tool", "{}");
/// # }
/// ```
pub fn prepare<H: Host>(
    session: &H::Session,
    call_id: &str,
    name: &str,
    arguments: &str,
) -> Result<serde_json::Value, String> {
    let input = serde_json::from_str(arguments)
        .map_err(|error| format!("Failed to parse arguments: {error}"))?;
    let cwd = H::directory(session).unwrap_or_else(|| std::path::Path::new("."));
    let input = H::enrich_input(&input, cwd, session);
    Ok(crate::enrich::<H>(name, call_id, session, input))
}
