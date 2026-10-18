use super::View;
use codetether_approval_core::ApprovalStore;
use codetether_approval_queue as approval_queue;

/// Apply one explicit approval command.
/// # Arguments
/// * `view` — Frontend status, input and presentation bindings.
/// * `prompt` — Approval command text.
/// # Returns
/// Whether the command was recognized, including reported errors.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_approval_view::command::View) {
/// let _ = codetether_approval_view::command::run(view, "/approve");
/// # }
/// ```
pub fn run(view: &mut impl View, prompt: &str) -> bool {
    let Some(action) = codetether_approval_core::command::Action::parse(prompt) else {
        return false;
    };
    match ApprovalStore::open_default() {
        Ok(store) => run_with_store(view, &store, action),
        Err(error) => {
            view.set_status(format!("Approval store unavailable: {error}"));
            true
        }
    }
}

fn run_with_store(
    view: &mut impl View,
    store: &ApprovalStore,
    action: codetether_approval_core::command::Action<'_>,
) -> bool {
    let Some(id) = target_id(action.id) else {
        view.set_status("No pending approval request".to_string());
        return true;
    };
    match codetether_approval_core::command::record(store, &id, &action) {
        Ok(stored) => super::result::decided(
            view,
            &stored.id,
            action.intent,
            &action.reason,
            codetether_approval_core::command::decision_text(&action, &stored),
        ),
        Err(error) => view.set_status(format!("Approval decision failed: {error}")),
    }
    true
}

fn target_id(id: Option<&str>) -> Option<String> {
    id.map(str::to_string)
        .or_else(approval_queue::active_id)
        .or_else(codetether_approval_core::interaction::pending_query::latest_id)
}
