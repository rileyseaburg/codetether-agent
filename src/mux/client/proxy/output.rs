//! Dedicated long-poll connection for ordered PTY output delivery.

use anyhow::{Result, bail};
use tokio::sync::mpsc;

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::super::connection::MuxConnection;

pub(super) struct Event {
    pub data: Vec<u8>,
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
                .request(ClientRequest::ReadProgram {
                    window_id: id,
                    offset,
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
                        exited: !running && data.is_empty(),
                        data,
                    })
                }
                ServerResponse::Error { message } => bail!(message),
                _ => bail!("mux server returned an invalid PTY output response"),
            });
            let finished = match &result {
                Ok(event) => event.exited,
                Err(_) => true,
            };
            if sender.send(result).await.is_err() || finished {
                break;
            }
        }
    });
    receiver
}
