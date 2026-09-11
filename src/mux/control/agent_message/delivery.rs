//! Verified TUI message delivery through a session's mux-owned terminal.

use anyhow::{Result, bail};

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};
use crate::mux::registry::SessionTarget;

pub(super) async fn submit_idle(target: &SessionTarget, message: &str) -> Result<()> {
    let mut connection = MuxConnection::connect(target).await?;
    input(&mut connection, target, super::terminal_text(message)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    input(&mut connection, target, super::terminal_submit()).await?;
    Ok(())
}

pub(super) async fn steer_active(target: &SessionTarget, message: &str) -> Result<()> {
    let mut connection = MuxConnection::connect(target).await?;
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Steer {
                text: message.to_string(),
            },
        })
        .await?;
    if !matches!(response, ServerResponse::Acknowledged) {
        bail!("mux rejected active session steering");
    }
    Ok(())
}

async fn input(
    connection: &mut MuxConnection,
    target: &SessionTarget,
    data: Vec<u8>,
) -> Result<()> {
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: target.active_window()?,
                data,
            },
        })
        .await?;
    if !matches!(response, ServerResponse::Acknowledged) {
        bail!("mux TUI rejected terminal input");
    }
    Ok(())
}
