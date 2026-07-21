//! Verified idle-TUI submission through its mux-owned terminal.

use anyhow::{Result, bail};

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};
use crate::mux::registry::MuxRecord;

pub(super) async fn submit_idle(record: &MuxRecord, message: &str) -> Result<()> {
    let mut connection = MuxConnection::connect(record).await?;
    input(&mut connection, record, super::terminal_text(message)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    input(&mut connection, record, super::terminal_submit()).await?;
    Ok(())
}

pub(super) async fn steer_active(record: &MuxRecord, message: &str) -> Result<()> {
    let mut connection = MuxConnection::connect(record).await?;
    if connection.version() < 7 {
        input(&mut connection, record, super::terminal_text(message)).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        return input(&mut connection, record, super::terminal_submit()).await;
    }
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

async fn input(connection: &mut MuxConnection, record: &MuxRecord, data: Vec<u8>) -> Result<()> {
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Input {
                window_id: record.state.active_window,
                data,
            },
        })
        .await?;
    if !matches!(response, ServerResponse::Acknowledged) {
        bail!("mux TUI rejected terminal input");
    }
    Ok(())
}
