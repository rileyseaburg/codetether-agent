use crate::approval::ApprovalStore;
use crate::tui::app::state::App;
use crate::tui::app::state::approval_queue;

pub(super) fn run(app: &mut App, prompt: &str) -> bool {
    let Some(action) = super::parse::Action::parse(prompt) else {
        return false;
    };
    match ApprovalStore::open_default() {
        Ok(store) => run_with_store(app, &store, action),
        Err(error) => {
            app.state.status = format!("Approval store unavailable: {error}");
            true
        }
    }
}

fn run_with_store(app: &mut App, store: &ApprovalStore, action: super::parse::Action<'_>) -> bool {
    let Some(id) = target_id(action.id) else {
        app.state.status = "No pending approval request".to_string();
        return true;
    };
    match super::store::record(store, &id, &action) {
        Ok(stored) => super::result::decided(
            app,
            &stored.id,
            action.intent,
            &action.reason,
            super::message::text(&action, &stored),
        ),
        Err(error) => app.state.status = format!("Approval decision failed: {error}"),
    }
    true
}

fn target_id(id: Option<&str>) -> Option<String> {
    id.map(str::to_string)
        .or_else(approval_queue::active_id)
        .or_else(crate::approval::live::latest_id)
}
