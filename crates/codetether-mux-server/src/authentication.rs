//! Authenticate one connection and validate its optional session binding.
use super::connection_error::error;
use super::context::ServerContext;
use anyhow::{Result, bail};
use codetether_mux_protocol::{ClientRequest, ServerResponse, VERSION, read_frame, write_frame};
use std::time::Duration;
use tokio::net::TcpStream;

pub(super) async fn accept(
    mut stream: &mut TcpStream,
    context: &ServerContext,
) -> Result<Option<String>> {
    let request = tokio::time::timeout(Duration::from_secs(5), read_frame(&mut stream)).await??;
    let Some(ClientRequest::Authenticate { token, session }) = request else {
        bail!("authentication required");
    };
    if !crate::token::matches(&token, &context.token) {
        write_frame(
            &mut stream,
            &super::connection_error::error("authentication failed"),
        )
        .await?;
        bail!("authentication failed");
    }
    if let Some(name) = &session
        && context.state.read().await.session(name).is_none()
    {
        write_frame(
            &mut stream,
            &error(&format!("unknown mux session '{name}'")),
        )
        .await?;
        bail!("unknown mux session");
    }
    write_frame(
        &mut stream,
        &ServerResponse::Authenticated { version: VERSION },
    )
    .await?;
    Ok(session)
}
