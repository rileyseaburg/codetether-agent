//! Record each result before a later context edit can refresh the session.
use crate::*;
use anyhow::Context;
pub(super) async fn execute<H: Host>(
    host: &H,
    session: &mut H::Session,
    calls: Vec<ContentPart>,
) -> anyhow::Result<()> {
    for part in calls {
        if let Some(result) = blocked::<H>(session, &part) {
            record_results::<H>(session, vec![part], vec![result]);
            continue;
        }
        let Some(call) = part.as_tool_call() else {
            continue;
        };
        let edits_context = codetether_tool_core::context_edit::is_context_edit(call.name);
        if edits_context {
            H::persist_session(session)
                .await
                .context("saving context before tool execution")?;
        }
        let result = host
            .execute_tool_for_session(session, call.id, call.name, call.arguments)
            .await;
        let refresh = if edits_context && result.success {
            H::refresh_session(session)
                .await
                .context("tool completed, but refreshing edited context failed")
        } else {
            Ok(())
        };
        record_results::<H>(session, vec![part], vec![result]);
        refresh?;
    }
    Ok(())
}
