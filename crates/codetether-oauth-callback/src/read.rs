//! OAuth callback read handling.
use anyhow::{Context, Result};
use tokio::io::AsyncReadExt;

pub(super) async fn read_http_request(stream: &mut tokio::net::TcpStream) -> Result<String> {
    let mut buffer = [0u8; 8192];
    let read = stream
        .read(&mut buffer)
        .await
        .context("Failed to read callback request")?;
    if read == 0 {
        anyhow::bail!("Callback request stream closed before data was received");
    }
    Ok(String::from_utf8_lossy(&buffer[..read]).to_string())
}
