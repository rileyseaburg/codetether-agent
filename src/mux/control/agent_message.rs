//! First-class agent messages delivered to a mux-owned TUI.

use anyhow::{Result, bail};
use serde_json::json;

#[path = "agent_message/delivery.rs"]
mod delivery;

/// Submit one message to the active TUI window of a named mux session.
///
/// # Errors
///
/// Returns an error unless the owning TUI semantically accepts the message.
pub(crate) async fn send_agent_message(name: &str, message: &str) -> Result<Option<String>> {
    let Some(record) = super::agent_target::load(name).await? else {
        return Ok(None);
    };
    let Some(runtime) = record.state.runtime.clone() else {
        bail!("mux session '{name}' has not registered semantic TUI state");
    };
    let transport = if runtime.processing {
        delivery::steer_active(&record, message).await?;
        "mux_steer"
    } else {
        delivery::submit_idle(&record, message).await?;
        "mux_tui"
    };
    Ok(Some(
        json!({
            "transport": transport,
            "session_title": runtime.session_title,
            "accepted": true,
            "delivery": "mux_server"
        })
        .to_string(),
    ))
}

pub(super) fn terminal_text(message: &str) -> Vec<u8> {
    message.replace(['\r', '\n'], " ").into_bytes()
}

pub(super) fn terminal_submit() -> Vec<u8> {
    b"\x1b[13u".to_vec()
}

#[cfg(test)]
#[test]
fn terminal_input_submits_one_tui_message() {
    assert_eq!(terminal_text("hello\nagent"), b"hello agent");
    assert_eq!(terminal_submit(), b"\x1b[13u");
}
