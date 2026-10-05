//! Summarize each closed 16-message chunk once, retaining an eight-message active tail.
use super::super::{capacity, ranges::Plan, registry_status, summarize, types::Snapshot};
use crate::session::index::{Granularity, SummaryRange};
use crate::session::store::{cursor, projection};
pub(super) async fn run(mut snapshot: Snapshot) {
    if let Err(error) = process(&mut snapshot).await {
        tracing::warn!(session_id = %snapshot.session_id, %error, "incremental RLM preparation failed");
    }
}
async fn process(snapshot: &mut Snapshot) -> anyhow::Result<()> {
    let Some(_permit) = capacity::acquire().await else {
        return Ok(());
    };
    while registry_status::is_current(&snapshot.session_id, snapshot.generation) {
        let Some(delivery) = cursor::next(&snapshot.session_id, "rlm", 24).await? else {
            break;
        };
        if delivery.session.messages.len() < 24 {
            break;
        }
        snapshot.messages = delivery.session.messages.into_vec();
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
        projection::commit(ticket, view).await?;
    }
    Ok(())
}
