//! Client for the active owner of a durable session.

use std::io::ErrorKind;
use std::path::Path;

use anyhow::{Result, ensure};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use super::wire::{Reply, Request};

pub(in crate::session::helper::steering) async fn send(
    session_id: &str,
    text: &str,
) -> Result<bool> {
    ensure!(!text.trim().is_empty(), "steering text is empty");
    ensure!(text.len() <= 64 * 1024, "steering text exceeds 64 KiB");
    send_to(&super::path::for_session(session_id)?, text).await
}

pub(super) async fn send_to(path: &Path, text: &str) -> Result<bool> {
    let stream = match UnixStream::connect(path).await {
        Ok(stream) => stream,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::NotFound | ErrorKind::ConnectionRefused
            ) =>
        {
            return Ok(false);
        }
        Err(error) => return Err(error.into()),
    };
    let (reader, mut writer) = stream.into_split();
    let mut request = serde_json::to_vec(&Request { text: text.into() })?;
    request.push(b'\n');
    writer.write_all(&request).await?;
    let mut response = String::new();
    BufReader::new(reader).read_line(&mut response).await?;
    Ok(serde_json::from_str::<Reply>(&response)?.accepted)
}
