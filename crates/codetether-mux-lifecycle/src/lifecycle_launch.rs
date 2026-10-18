//! Launch a semantic TUI inside a managed mux server.

use anyhow::{Result, bail};

use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest, ServerResponse};

pub(super) async fn tui(name: &str, session_id: Option<&str>) -> Result<()> {
    if session_id.is_some_and(|id| {
        !id.chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_'))
    }) {
        bail!("invalid durable session id");
    }
    let target = codetether_mux_registry::load(name).await?;
    let window = target
        .session()
        .map(|session| session.active_window)
        .ok_or_else(|| anyhow::anyhow!("mux session '{name}' is unavailable"))?;
    let command = session_id.map_or_else(
        || "codetether tui --yolo\n".to_string(),
        |id| format!("codetether tui --session {id} --yolo\n"),
    );
    let mut connection = MuxConnection::connect(&target).await?;
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: window,
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

pub(super) use super::lifecycle_wait::wait_runtime;
