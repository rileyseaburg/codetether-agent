//! Apply parallel read-only tool outputs to a session.

use std::path::Path;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::{ContentPart, Message, Provider, Role};
use crate::session::helper::runtime::is_codesearch_no_match_output;
use crate::session::{Session, SessionEvent};
use crate::tool::ToolRegistry;

pub(crate) async fn try_execute(
    session: &mut Session,
    calls: &[(String, String, serde_json::Value)],
    registry: &ToolRegistry,
    cwd: &Path,
    model: &str,
    provider: Arc<dyn Provider>,
    event_tx: &mpsc::Sender<SessionEvent>,
    no_match_count: &mut u32,
) -> bool {
    let Some(jobs) = super::eligibility::prepare(
        calls,
        cwd,
        session.metadata.model.as_deref(),
        &session.id,
        &session.agent,
        session.metadata.provenance.as_ref(),
    ) else {
        return false;
    };
    let _ = session.save().await;
    tracing::info!(
        count = jobs.len(),
        "Executing read-only tool batch in parallel"
    );
    for out in super::run::execute(jobs, registry, &session.id, event_tx).await {
        let content = super::route::route(session, model, Arc::clone(&provider), &out);
        session.add_message(Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: out.tool_id,
                content,
            }],
        });
        if is_codesearch_no_match_output(&out.tool_name, out.success, &out.content) {
            *no_match_count += 1;
        } else {
            *no_match_count = 0;
        }
    }
    true
}
