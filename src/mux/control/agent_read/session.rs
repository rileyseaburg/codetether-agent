//! Durable semantic transcript projection for a registered mux TUI.

use anyhow::Result;

use crate::mux::registry::MuxRecord;

#[path = "session/projection.rs"]
mod projection;

pub(super) async fn read(record: &MuxRecord) -> Result<Option<String>> {
    let Some(runtime) = record.state.runtime.as_ref() else {
        return Ok(None);
    };
    let Some(window) = record
        .state
        .windows
        .iter()
        .find(|window| window.id == record.state.active_window)
    else {
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
