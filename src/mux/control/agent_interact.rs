//! Explicitly submit input already waiting in a mux-owned TUI.

use anyhow::{Result, bail};
use serde_json::json;

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

pub(crate) async fn interact_agent(name: &str) -> Result<Option<String>> {
    let Some(record) = super::agent_target::load(name).await? else {
        return Ok(None);
    };
    let Some(runtime) = record.state.runtime.as_ref() else {
        bail!("mux session '{name}' has not registered semantic TUI state");
    };
    if !runtime.needs_interaction {
        bail!("session is not waiting: {}", runtime.session_title);
    }
    let mut connection = MuxConnection::connect(&record).await?;
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: record.state.active_window,
                data: super::agent_message::terminal_submit(),
            },
        })
        .await?;
    if !matches!(response, ServerResponse::Acknowledged) {
        bail!("mux TUI rejected the pending interaction");
    }
    Ok(Some(
        json!({"accepted": true, "delivery": "mux_server", "session_title": runtime.session_title})
            .to_string(),
    ))
}
