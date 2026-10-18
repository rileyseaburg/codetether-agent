//! Apply parallel read-only tool outputs to a session.

use tokio::sync::mpsc;

use crate::Host;
use codetether_prompt_policy::is_codesearch_no_match_output;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_events::SessionEvent;
use codetether_tool_execution::Host as ExecutionHost;

/// Prepare, execute and record a batch containing at least two eligible reads.
/// # Arguments
/// `session` supplies persistence and recording; `calls` is the proposed batch;
/// `registry` supplies execution gates; `event_tx` receives progress events;
/// `no_match_count` tracks consecutive empty code-search results in call order.
/// # Returns
/// False if the batch is ineligible or saving fails; true after recording it.
/// # Panics
/// Progress registration can panic if its shared registry lock was poisoned.
/// # Examples
/// ```no_run
/// # async fn example(session: &mut impl codetether_tool_parallel::Host,
/// # execution: &impl codetether_tool_execution::Host,
/// # tx: &tokio::sync::mpsc::Sender<codetether_session_events::SessionEvent<()>>) {
/// let mut misses = 0;
/// assert!(!codetether_tool_parallel::try_execute(
///     session, &[], execution, tx, &mut misses
/// ).await);
/// # }
/// ```
pub async fn try_execute<S: Send + 'static>(
    session: &mut impl Host,
    calls: &[(String, String, serde_json::Value)],
    registry: &impl ExecutionHost,
    event_tx: &mpsc::Sender<SessionEvent<S>>,
    no_match_count: &mut u32,
) -> bool {
    let Some(jobs) = super::prepare::prepare(calls, session) else {
        return false;
    };
    if session.save().await.is_err() {
        return false;
    }
    tracing::info!(target: "codetether_agent::session::helper::tool_modules::tool_parallel::record",
        count = jobs.len(),
        "Executing read-only tool batch in parallel"
    );
    for out in super::run::execute(jobs, registry, session.id(), event_tx).await {
        let content = session.route(&out);
        let mut parts = vec![ContentPart::ToolResult {
            tool_call_id: out.tool_id,
            content,
        }];
        parts.extend(out.images);
        session.record(Message {
            role: Role::Tool,
            content: parts,
        });
        if is_codesearch_no_match_output(&out.tool_name, out.success, &out.content) {
            *no_match_count += 1;
        } else {
            *no_match_count = 0;
        }
    }
    true
}
