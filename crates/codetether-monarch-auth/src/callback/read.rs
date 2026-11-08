//! Bounded callback request reads; the caller supplies the read deadline.
use anyhow::{Result, anyhow, ensure};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub(super) async fn read_request(stream: &mut TcpStream) -> Result<String> {
    let mut bytes = Vec::new();
    let mut chunk = [0; 1024];
    loop {
        let count = stream
            .read(&mut chunk)
            .await
            .map_err(|_| anyhow!("Callback read failed"))?;
        ensure!(
            count != 0 && bytes.len() + count <= 8192,
            "Invalid callback request size"
        );
        bytes.extend_from_slice(&chunk[..count]);
        if bytes.windows(4).any(|w| w == b"\r\n\r\n") {
            return String::from_utf8(bytes).map_err(|_| anyhow!("Invalid callback encoding"));
        }
    }
}
