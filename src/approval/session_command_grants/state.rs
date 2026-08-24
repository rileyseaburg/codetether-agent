//! Synchronized storage for session-scoped command prefixes.

use super::store::state;

pub(super) fn remember(
    id: &str,
    prefixes: Vec<String>,
    session_id: Option<&str>,
    workspace: Option<&str>,
) {
    if let (Some(session), Some(workspace)) = (
        super::normalized_scope(session_id),
        super::normalized_scope(workspace),
    ) {
        state()
            .requests
            .insert(id.to_string(), (session.into(), workspace.into(), prefixes));
    }
}

pub(super) fn grant(id: &str) {
    let mut state = state();
    if let Some((session, workspace, prefixes)) = state.requests.remove(id) {
        state.allowed.extend(
            prefixes
                .into_iter()
                .map(|prefix| (session.clone(), workspace.clone(), prefix)),
        );
    }
}

pub(super) fn discard(id: &str) {
    state().requests.remove(id);
}

pub(super) fn allowed(command: &str, session: Option<&str>, workspace: Option<&str>) -> bool {
    let (Some(session), Some(workspace)) = (
        super::normalized_scope(session),
        super::normalized_scope(workspace),
    ) else {
        return false;
    };
    let command = command.trim_start();
    state()
        .allowed
        .iter()
        .filter(|(grant_session, grant_workspace, _)| {
            grant_session == session && grant_workspace == workspace
        })
        .any(|(_, _, prefix)| command == prefix || command.starts_with(&format!("{prefix} ")))
}

#[cfg(test)]
pub(super) fn reset() {
    *state() = super::store::State::default();
}
