//! Receive one valid loopback authorization result within a bounded deadline.
use super::{Callback, parse::parse, read::read_request, reply::reply};
use anyhow::{Result, anyhow, bail};
use std::time::Duration;
use tokio::{net::TcpListener, time::timeout};

pub(crate) async fn receive(listener: TcpListener, state: &str) -> Result<String> {
    let authority = listener
        .local_addr()
        .map_err(|_| anyhow!("Loopback address unavailable"))?
        .to_string();
    timeout(Duration::from_secs(300), async {
        loop {
            let (mut stream, peer) = listener
                .accept()
                .await
                .map_err(|_| anyhow!("Callback listener failed"))?;
            if !peer.ip().is_loopback() {
                continue;
            }
            let parsed = match timeout(Duration::from_secs(3), read_request(&mut stream)).await {
                Ok(Ok(request)) => parse(&request, &authority, state),
                _ => continue,
            };
            reply(&mut stream, &parsed).await;
            match parsed {
                Ok(Callback::Code(code)) => return Ok(code),
                Ok(Callback::Denied) => bail!("Monarch authorization was denied"),
                Err(_) => continue,
            }
        }
    })
    .await
    .map_err(|_| anyhow!("Monarch login timed out; retry codetether auth monarch login"))?
}
