//! Live event and transcript insertion for a synthetic result.
use super::Host;
use crate::Call;
/// Emit and append a synthetic result, without completion-only persistence.
///
/// # Arguments
/// `host` supplies destinations; `call` identifies the request; `content` and
/// `success` describe its synthetic result.
///
/// # Returns
/// Completes after the live event and transcript insertion.
///
/// # Examples
/// ```no_run
/// # async fn reject(host: &mut impl codetether_prompt_loop::tool_publication::Host,
/// # call: &codetether_prompt_loop::Call) {
/// codetether_prompt_loop::tool_publication::record(
///     host, call, "Request declined".into(), false,
/// ).await;
/// # }
/// ```
pub async fn record(host: &mut impl Host, call: &Call, content: String, success: bool) {
    if let Some(tx) = host.events() {
        codetether_tool_reporting::complete(tx, &call.id, &call.name, content.clone(), success, 0)
            .await;
    }
    host.append(codetether_tool_core::messages::tool_result_with_status(
        call.id.clone(),
        &call.name,
        success,
        content,
    ));
}
