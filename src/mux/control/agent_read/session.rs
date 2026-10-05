//! Durable semantic transcript projection for a registered mux TUI.

use anyhow::Result;

use crate::mux::registry::SessionTarget;

#[path = "session/projection.rs"]
mod projection;

pub(super) async fn read(target: &SessionTarget) -> Result<Option<String>> {
    let Some(runtime) = target.runtime() else {
        return Ok(None);
    };
    if let Some(indexed) = crate::session::store::projection::read(&runtime.session_id, "recall").await? {
        return Ok(Some(projection::render(runtime, &indexed)));
    }
    let Some(window) = target.active() else {
        return Ok(None);
    };
    let path = window
        .workspace
        .join(".codetether-agent/sessions/recall/sessions")
        .join(format!("{}.json", runtime.session_id));
    let Ok(data) = tokio::fs::read(path).await else {
        return Ok(None);
    };
    let indexed = serde_json::from_slice(&data)?;
    Ok(Some(projection::render(runtime, &indexed)))
}

#[cfg(test)]
#[path = "session/tests.rs"]
mod tests;