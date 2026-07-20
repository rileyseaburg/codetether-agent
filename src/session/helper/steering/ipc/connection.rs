//! One session-steering socket exchange.

use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use super::wire::{Reply, Request};
use crate::session::helper::steering::SteeringInput;

pub(super) async fn handle(stream: UnixStream, session_id: String) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();
    let Some(line) = lines.next_line().await? else {
        return Ok(());
    };
    let request: Request = serde_json::from_str(&line)?;
    let accepted = !request.text.trim().is_empty()
        && super::super::push(&session_id, SteeringInput::new(request.text, vec![]));
    let mut response = serde_json::to_vec(&Reply { accepted })?;
    response.push(b'\n');
    writer.write_all(&response).await?;
    Ok(())
}
