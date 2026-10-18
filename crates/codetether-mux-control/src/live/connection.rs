//! Authenticated event-driven reads from one mux PTY.

use anyhow::{Result, bail};
use tokio::sync::mpsc::Sender;

use super::MuxLiveOutput;
use codetether_mux_client::MuxConnection;
use codetether_mux_protocol::{ClientRequest, ProgramRequest, ServerResponse};

pub(super) async fn run(name: &str, sender: &Sender<MuxLiveOutput>) -> Result<()> {
    let target = codetether_mux_registry::load(name).await?;
    let window_id = target.active_window()?;
    let mut connection = MuxConnection::connect(&target).await?;
    let response = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Tail { window_id },
        })
        .await?;
    let ServerResponse::ProgramAttached { mut offset, .. } = response else {
        bail!("mux session {name} has no output tail");
    };
    loop {
        let response = connection
            .request(ClientRequest::Program {
                request: ProgramRequest::Read { window_id, offset },
            })
            .await?;
        let ServerResponse::ProgramOutput {
            data,
            next_offset,
            running,
        } = response
        else {
            bail!("mux session {name} returned invalid live output");
        };
        offset = next_offset;
        if !data.is_empty() {
            super::output::send(sender, name, data, offset, running).await?;
        }
        if !running {
            return Ok(());
        }
    }
}
