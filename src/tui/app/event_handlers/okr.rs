//! OKR approval and denial handlers for the TUI.
//!
//! Processes the pending OKR approval stored in
//! [`AppState::pending_okr_approval`] when the user presses
//! the approve or deny key binding. Saves the decision,
//! updates chat state, and starts relay execution on approval.

use std::sync::Arc;

use crate::okr::ApprovalDecision;
use crate::tui::app::autochat::worker::start_autochat_relay;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::okr_save::spawn_okr_save;

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

    let rx = start_autochat_relay(task.clone(), model.clone());
    app.state.autochat.running = true;
    app.state.autochat.rx = Some(rx);
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        format!(
            "✅ OKR approved. Starting OKR-gated relay (ID: {okr_id})...\nTask: {task}\nAgents: {agent_count} | Model: {model}"
        ),
    ));
    app.state.scroll_to_bottom();
    app.state.status = format!("OKR approved (ID: {okr_id}). Relay started.");
}

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
        "❌ OKR denied. Relay cancelled.".to_string(),
    ));
    app.state.scroll_to_bottom();
    app.state.status = "OKR denied. Relay cancelled.".to_string();
}
