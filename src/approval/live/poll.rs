//! Durable-store polling for decisions made by another process.

#[path = "poll_session.rs"]
mod session;

use tokio::sync::oneshot;

use super::LiveApprovalDecision;

pub(super) async fn wait(
    id: &str,
    mut receiver: oneshot::Receiver<LiveApprovalDecision>,
    events: &tokio::sync::mpsc::Sender<crate::session::SessionEvent>,
    tool_call_id: &str,
    tool: &str,
) -> LiveApprovalDecision {
    loop {
        tokio::select! {
            decision = &mut receiver => {
                return decision.unwrap_or_else(|_| LiveApprovalDecision::denied());
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                match stored(id) {
                    Ok(Some(decision)) => {
                        super::decision_event::emit(events, tool_call_id, tool, id);
                        return decision;
                    }
                    Ok(None) => {}
                    Err(error) => return LiveApprovalDecision::denied_with(error.to_string()),
                }
            }
        }
    }
}

fn stored(id: &str) -> anyhow::Result<Option<LiveApprovalDecision>> {
    let store = crate::approval::ApprovalStore::open_default()?;
    let Some(decision) = store.decision(id)? else {
        return Ok(None);
    };
    match decision.status {
        crate::approval::ApprovalStatus::Approved => {
            session::settle(&store, id, &decision)?;
            Ok(Some(LiveApprovalDecision::Approved))
        }
        crate::approval::ApprovalStatus::Denied => Ok(Some(LiveApprovalDecision::Denied {
            reason: Some(decision.reason),
        })),
        crate::approval::ApprovalStatus::Pending => Ok(None),
    }
}
