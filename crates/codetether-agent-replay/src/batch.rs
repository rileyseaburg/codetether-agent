//! Batch mode selection and ordered sequential execution.
use crate::*;
/// Execute and record a batch using native policy and dispatch.
/// # Arguments
/// Mutable session and ordered model-emitted tool calls.
/// # Returns
/// Returns after all selected execution results are recorded.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_replay::*;
/// # async fn example<H: Host>(host: &H, session: &mut H::Session) {
/// execute(host,session,vec![]).await;
/// # }
/// ```
pub async fn execute<H: Host>(host: &H, session: &mut H::Session, tool_calls: Vec<ContentPart>) {
    let results = if can_parallelize::<H>(session, &tool_calls) {
        crate::parallel::execute(host, session, &tool_calls).await
    } else {
        execute_sequential(host, session, &tool_calls).await
    };
    record_results::<H>(session, tool_calls, results);
}

async fn execute_sequential<H: Host>(
    host: &H,
    session: &H::Session,
    calls: &[ContentPart],
) -> Vec<ToolResult> {
    let mut out = Vec::with_capacity(calls.len());
    for part in calls {
        if let Some(result) = blocked::<H>(session, part) {
            out.push(result);
            continue;
        }
        let Some(call) = part.as_tool_call() else {
            continue;
        };
        out.push(
            host.execute_tool_for_session(session, call.id, call.name, call.arguments)
                .await,
        );
    }
    out
}
