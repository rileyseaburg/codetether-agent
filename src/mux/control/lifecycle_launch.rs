//! Launch a semantic TUI inside a managed mux server.

use anyhow::{Result, bail};

use super::MuxSessionSummary;
use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

pub(super) async fn tui(name: &str, session_id: Option<&str>) -> Result<()> {
    if session_id.is_some_and(|id| {
        !id.chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_'))
    }) {
        bail!("invalid durable session id");
    }
    let record = crate::mux::registry::load(name).await?;
    let command = session_id.map_or_else(
        || "codetether tui --yolo\n".to_string(),
        |id| format!("codetether tui --session {id} --yolo\n"),
    );
    let mut connection = MuxConnection::connect(&record).await?;
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: record.state.active_window,
                data: command.into_bytes(),
            },
        })
        .await?;
    if matches!(response, ServerResponse::Acknowledged) {
        Ok(())
    } else {
        bail!("mux rejected TUI launch")
    }
}

pub(super) async fn wait_runtime(name: &str) -> Result<MuxSessionSummary> {
    for _ in 0..200 {
        let sessions = super::list_sessions().await?;
        if let Some(session) = sessions
            .into_iter()
            .find(|item| item.name == name && item.runtime.is_some())
        {
            return Ok(session);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    bail!("mux TUI did not register semantic state")
}
