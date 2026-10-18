//! Pending-objective approval and denial transitions.
use std::sync::Arc;

use codetether_okr::ApprovalDecision;

use super::Host;

use super::save::spawn_okr_save;

/// Approve the pending objective and start its relay.
/// # Arguments
/// * `host` — Pending approval state and frontend effects.
/// # Returns
/// Updates the host when a pending objective exists; otherwise does nothing.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_relay_okr::interactive::approval::Host) {
/// codetether_relay_okr::interactive::approval::handle_okr_approve(host).await;
/// # }
/// ```
pub async fn handle_okr_approve<H: Host>(host: &mut H) {
    let Some(pending) = host.pending().take() else {
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

    if let Some(repo) = host.repository() {
        spawn_okr_save(Arc::clone(repo), pending.okr, approved_run);
    }

    let rx = H::start_relay(task.clone(), model.clone());
    host.set_relay_running(true);
    host.set_relay_receiver(rx);
    host.push_system(format!(
            "✅ OKR approved. Starting OKR-gated relay (ID: {okr_id})...\nTask: {task}\nAgents: {agent_count} | Model: {model}"
        ));
    host.scroll_to_bottom();
    host.set_status(format!("OKR approved (ID: {okr_id}). Relay started."));
}

/// Deny and clear the pending objective.
/// # Arguments
/// * `host` — Pending approval state and frontend effects.
/// # Returns
/// Updates the host when a pending objective exists; otherwise does nothing.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_relay_okr::interactive::approval::Host) {
/// codetether_relay_okr::interactive::approval::handle_okr_deny(host).await;
/// # }
/// ```
pub async fn handle_okr_deny<H: Host>(host: &mut H) {
    let Some(mut pending) = host.pending().take() else {
        return;
    };

    pending.run.record_decision(ApprovalDecision::deny(
        pending.run.id,
        "User denied via TUI",
    ));

    host.push_system("❌ OKR denied. Relay cancelled.".to_string());
    host.scroll_to_bottom();
    host.set_status("OKR denied. Relay cancelled.".to_string());
}
