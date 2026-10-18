//! Dedicated long-poll connection for ordered PTY output delivery.

use anyhow::Result;
use tokio::sync::mpsc;

use codetether_mux_protocol::{ClientRequest, ProgramRequest};

use super::output_event::Event;
use codetether_mux_client::MuxConnection;

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
            let result =
                response.and_then(|response| super::output_event::decode(response, &mut offset));
            let finished = result.as_ref().map_or(true, |event| event.exited);
            if sender.send(result).await.is_err() || finished {
                break;
            }
        }
    });
    receiver
}
