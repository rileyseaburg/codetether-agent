//! Ordered synthetic failure and retry insertion.
use super::Host;
use codetether_provider_core::Message;
/// Repair truncated tool-call output and report whether the provider should retry.
/// # Arguments
/// * `host` — Native session and event bindings.
/// * `response` — Assistant message containing the truncated calls.
/// * `calls_empty` — Whether no complete executable calls remain.
/// * `truncated` — Ordered call IDs and tool names that could not be parsed.
/// # Returns
/// True after retry insertion and output reset for an entirely truncated turn.
/// # Examples
/// ```no_run
/// # async fn repair<H: codetether_prompt_loop::repair::Host>(host: &mut H, message: &codetether_provider_core::Message) {
/// let retry = codetether_prompt_loop::repair::record(host, message, true, &[]).await;
/// assert!(!retry);
/// # }
/// ```
pub async fn record(
    host: &mut impl Host,
    response: &Message,
    calls_empty: bool,
    truncated: &[(String, String)],
) -> bool {
    if truncated.is_empty() {
        return false;
    }
    if calls_empty {
        host.add_message(response.clone());
    }
    for (id, name) in truncated {
        let content = codetether_tool_recovery::error_content(name);
        if let Some(tx) = host.events() {
            codetether_tool_reporting::complete(tx, id, name, content.clone(), false, 0).await;
        }
        host.add_message(codetether_tool_core::messages::tool_result_with_status(
            id.clone(),
            name,
            false,
            content,
        ));
    }
    if calls_empty {
        host.add_message(codetether_tool_recovery::retry_prompt(truncated));
        host.clear_output();
        true
    } else {
        false
    }
}
