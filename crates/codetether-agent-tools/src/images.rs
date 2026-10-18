//! Apply shared image argument enrichment to borrowed session history.
use crate::*;
/// Enrich image-tool input using the current conversation history.
/// # Arguments
/// Tool name, call ID, session and parsed input.
/// # Returns
/// The shared image-enrichment result.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_tools::*;
/// # async fn example<H: Host>(host: &H, session: &H::Session) {
/// let _ = enrich::<H>("tool", "call-1", session, serde_json::json!({}));
/// # }
/// ```
pub fn enrich<H: Host>(name: &str, call_id: &str, session: &H::Session, input: Value) -> Value {
    codetether_agent_core::execution::images::enrich(name, call_id, H::messages(session), input)
}
