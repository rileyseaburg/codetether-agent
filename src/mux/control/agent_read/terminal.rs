//! Bounded raw-terminal fallback for mux sessions without semantic state.

use anyhow::{Result, bail};
use serde_json::json;

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ProgramRequest, ServerResponse};
use crate::mux::registry::MuxRecord;

pub(super) async fn read(name: &str, record: &MuxRecord) -> Result<String> {
    let window_id = record.state.active_window;
    let mut connection = MuxConnection::connect(record).await?;
    let tail = connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Tail { window_id },
        })
        .await?;
    let ServerResponse::ProgramAttached {
        offset,
        replay_until,
        ..
    } = tail
    else {
        bail!("mux server returned an invalid tail response");
    };
    if offset == replay_until {
        return Ok(json!({"transport":"mux","session":name,"output":""}).to_string());
    }
    match connection
        .request(ClientRequest::Program {
            request: ProgramRequest::Read { window_id, offset },
        })
        .await?
    {
        ServerResponse::ProgramOutput {
            data,
            next_offset,
            running,
        } => Ok(json!({
            "transport":"mux","session":name,"output":super::text::bounded(&data),
            "offset":next_offset,"running":running
        })
        .to_string()),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid output response"),
    }
}
