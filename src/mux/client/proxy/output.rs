//! Dedicated long-poll connection for ordered PTY output delivery.

use anyhow::{Result, bail};
use tokio::sync::mpsc;

use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};

use super::super::connection::MuxConnection;

pub(super) struct Event {
    pub data: Vec<u8>,
    pub next_offset: u64,
    pub exited: bool,
}

pub(super) fn start(
    mut connection: MuxConnection,
    id: u64,
    mut offset: u64,
) -> mpsc::Receiver<Result<Event>> {
    let (sender, receiver) = mpsc::channel(1);
    tokio::spawn(async move {
        loop {
            let response = connection
                .request(ClientRequest::Program {
                    request: ProgramRequest::Read {
                        window_id: id,
                        offset,
                    },
                })
                .await;
            let result = response.and_then(|response| match response {
                ServerResponse::ProgramOutput {
                    data,
                    next_offset,
                    running,
                } => {
                    offset = next_offset;
                    Ok(Event {
                        next_offset,
                        exited: !running && data.is_empty(),
                        data,
                    })
                }
                ServerResponse::Error { message } => bail!(message),
                _ => bail!("mux server returned an invalid PTY output response"),
            });
            let finished = result.as_ref().map_or(true, |event| event.exited);
            if sender.send(result).await.is_err() || finished {
                break;
            }
        }
    });
    receiver
}
