//! Spawn the reviewer agent for a pending approval (advise mode).
//!
//! Runs alongside the LSP analysis. The verdict lands on the queue item via
//! `approval_queue::set_review`; if the human decides first the item is gone
//! and the late verdict is dropped harmlessly.

use std::path::PathBuf;

use crate::review::{ReviewSubject, runtime};
use crate::session::tasks::{TaskLog, TaskState, governance_block};
use crate::tui::app::state::{App, approval_queue, approval_queue::ApprovalSnapshot};

pub(super) fn start(app: &mut App, item: &ApprovalSnapshot) {
    let Some(review) = runtime::enabled() else {
        return;
    };
    if !crate::tool::proposed_content::supported(&item.tool) {
        return;
    }
    let workspace = PathBuf::from(&app.state.cwd_display);
    let session_id = app.state.session_id.clone();
    // `[review].model` wins; otherwise reuse whichever model is answering
    // this session so the reviewer never needs its own credentials.
    let session_model = app.state.last_completion_model.clone();
    let id = item.id.clone();
    let mut subject = ReviewSubject {
        tool: item.tool.clone(),
        action: item.action.clone(),
        resource: item.resource.clone(),
        justification: item.justification.clone(),
        preview: item.preview.clone(),
        goal: None,
    };
    approval_queue::set_review(&id, None);
    tokio::spawn(async move {
        subject.goal = goal_block(session_id.as_deref()).await;
        let verdict = crate::review::review(
            &review.config,
            &review.registry,
            session_model.as_deref(),
            workspace,
            subject,
        )
        .await;
        tracing::info!(approval = %id, outcome = verdict.outcome.label(), "Reviewer verdict");
        approval_queue::set_review(&id, Some(verdict));
    });
}

async fn goal_block(session_id: Option<&str>) -> Option<String> {
    let log = TaskLog::for_session(session_id?).ok()?;
    let events = log.read_all().await.ok()?;
    governance_block(&TaskState::from_log(&events))
}
