//! OKR approval and denial handlers for the TUI.
//!
//! Processes the pending OKR approval stored in
//! [`AppState::pending_okr_approval`] when the user presses
//! the approve or deny key binding.  Saves the decision to
//! the OKR repository and updates the chat messages.
//!
//! # Examples
//!
//! ```ignore
//! handle_okr_approve(&mut app).await;
//! handle_okr_deny(&mut app).await;
//! ```

use std::sync::Arc;

use crate::okr::ApprovalDecision;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::okr_save::spawn_okr_save;

/// Approve the pending OKR and start the relay.
///
/// Takes the pending approval from app state, records the
/// decision, persists to the OKR repository, and pushes a
/// confirmation message into the chat.
///
/// # Examples
///
/// ```ignore
/// handle_okr_approve(&mut app).await;
/// ```
#[allow(dead_code)]
pub(super) async fn handle_okr_approve(app: &mut App) {
    let Some(pending) = app.state.pending_okr_approval.take() else {
        return;
    };

    let okr_id = pending.okr.id;
    let task = pending.task.clone();
    let agent_count = pending.agent_count;
    let model = pending.model.clone();

    let mut approved_run = pending.run;
    approved_run.record_decision(ApprovalDecision::approve(
        approved_run.id,
        "User approved via TUI",
    ));

    if let Some(ref repo) = app.state.okr_repository {
        spawn_okr_save(Arc::clone(repo), pending.okr, approved_run);
    }

    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!(
            "\u{2705} OKR approved. Starting OKR-gated relay (ID: {okr_id})...\n\
                 Task: {task}\n\
                 Agents: {agent_count} | Model: {model}"
        ),
    ));
    app.state.scroll_to_bottom();
    app.state.status = format!("OKR approved (ID: {okr_id}). Relay starting...");
}

/// Deny the pending OKR and cancel the relay.
///
/// Records the denial decision in the run metadata and
/// pushes a cancellation message into the chat.
///
/// # Examples
///
/// ```ignore
/// handle_okr_deny(&mut app).await;
/// ```
#[allow(dead_code)]
pub(super) async fn handle_okr_deny(app: &mut App) {
    let Some(mut pending) = app.state.pending_okr_approval.take() else {
        return;
    };

    pending.run.record_decision(ApprovalDecision::deny(
        pending.run.id,
        "User denied via TUI",
    ));

    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        "\u{274c} OKR denied. Relay cancelled.".to_string(),
    ));
    app.state.scroll_to_bottom();
    app.state.status = "OKR denied. Relay cancelled.".to_string();
}
