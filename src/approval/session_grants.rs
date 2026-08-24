//! Process-local approval grants for the current interactive session.

use super::ApprovalReceipt;

#[path = "session_grants/state.rs"]
mod state;

pub(crate) fn remember_request(id: &str, session_id: Option<&str>) {
    state::remember(id, session_id);
}

pub(crate) fn discard_request(id: &str) {
    state::discard(id);
}

pub fn grant(receipt: &ApprovalReceipt) {
    state::grant(receipt);
}

pub(crate) fn allowed_scoped(
    tool: &str,
    action: &str,
    resource: &str,
    session_id: Option<&str>,
) -> bool {
    state::allowed(tool, action, resource, session_id)
}

fn normalized_session(session: Option<&str>) -> Option<&str> {
    session.map(str::trim).filter(|value| !value.is_empty())
}

#[cfg(test)]
pub(crate) fn reset() {
    state::reset();
}
