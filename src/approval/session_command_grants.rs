//! Process-local command-prefix grants for the current session.

#[path = "session_command_grants/state.rs"]
mod state;
#[path = "session_command_grants/store.rs"]
mod store;

pub(crate) fn remember_scoped_request_in(
    id: &str,
    prefixes: Vec<String>,
    session_id: Option<&str>,
    workspace: Option<&str>,
) {
    let prefixes = prefixes
        .into_iter()
        .filter_map(|p| normalize(&p))
        .collect::<Vec<_>>();
    if prefixes.is_empty() {
        return;
    }
    state::remember(id, prefixes, session_id, workspace);
}

pub(crate) fn discard_request(id: &str) {
    state::discard(id);
}

pub fn grant_for_request(id: &str) {
    state::grant(id);
}

pub(crate) fn allowed_scoped_in(
    command: &str,
    session_id: Option<&str>,
    workspace: Option<&str>,
) -> bool {
    state::allowed(command, session_id, workspace)
}

fn normalize(prefix: &str) -> Option<String> {
    let prefix = prefix.trim();
    (!prefix.is_empty() && !prefix.contains(['\n', '\r'])).then(|| prefix.to_string())
}

fn normalized_scope(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

#[cfg(test)]
pub(crate) fn reset() {
    state::reset();
}
