//! Summarize each closed 16-message chunk once, retaining an eight-message active tail.
use crate::summarize;
use codetether_context_policy::summary_index::{Granularity, SummaryRange};
use codetether_proactive_planning::Plan;
use codetether_proactive_runtime as registry_status;
use codetether_proactive_runtime::{self as capacity, Snapshot};
pub(super) async fn run<H: crate::Host>(mut snapshot: Snapshot, host: &H) {
    if let Err(error) = process(&mut snapshot, host).await {
        tracing::warn!(target: "codetether_agent::session::index_produce::proactive::worker::incremental", session_id = %snapshot.session_id, %error, "incremental RLM preparation failed");
    }
}
async fn process<H: crate::Host>(snapshot: &mut Snapshot, host: &H) -> anyhow::Result<()> {
    let Some(_permit) = capacity::acquire().await else {
        return Ok(());
    };
    while registry_status::is_current(&snapshot.session_id, snapshot.generation) {
        let Some(delivery) = host.next(&snapshot.session_id, "rlm", 24).await? else {
            break;
        };
        if H::message_count(&delivery.session) < 24 {
            break;
        }
        snapshot.messages = H::into_messages(delivery.session);
        let plan = Plan {
            range: SummaryRange { start: 0, end: 16 },
            granularity: Granularity::Phase,
            target_tokens: 512,
        };
        let node = summarize::range(snapshot, plan).await?;
        let mut ticket = delivery.ticket;
        ticket.to = ticket.from + 16;
        let view = serde_json::json!({ "documents": [{
            "start": ticket.from, "end": ticket.to, "node": node
        }] });
        host.commit(ticket, view).await?;
    }
    Ok(())
}
