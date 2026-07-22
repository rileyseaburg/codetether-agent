//! JSON projection for one authoritative mux-session record.

use serde_json::{Value, json};

use crate::mux::MuxRuntimeStatus;
use crate::mux::control::MuxSessionSummary;

pub(super) fn project(session: MuxSessionSummary) -> Value {
    let runtime = session.runtime.as_ref();
    let principal = runtime.map(|item| &item.principal);
    let windows = session
        .windows
        .into_iter()
        .map(|window| {
            json!({
                "id": window.id,
                "title": window.title,
                "workspace": window.workspace,
            })
        })
        .collect::<Vec<_>>();
    json!({
        "name": session.name,
        "pid": session.pid,
        "active_window": session.active_window,
        "windows": windows,
        "reachable": session.reachable,
        "runtime": runtime.map(project_runtime),
        "agent_name": principal.map(|item| &item.agent_name),
        "agent_identity_id": principal.and_then(|item| item.agent_identity_id.as_deref()),
        "persona_id": principal.and_then(|item| item.persona_id.as_deref()),
        "spiffe_id": principal.and_then(|item| item.spiffe_id.as_deref()),
        "provenance_id": principal.and_then(|item| item.provenance_id.as_deref()),
    })
}

fn project_runtime(runtime: &MuxRuntimeStatus) -> Value {
    json!({
        "session_title": runtime.session_title,
        "processing": runtime.processing,
        "message_count": runtime.message_count,
        "current_tool": runtime.current_tool,
        "needs_interaction": runtime.needs_interaction,
        "lagging": runtime.lagging,
        "principal": &runtime.principal,
    })
}

#[cfg(test)]
#[path = "session_projection_tests.rs"]
mod tests;
