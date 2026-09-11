//! Authentication and request loop for one mux client.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Result, bail};
use tokio::net::TcpStream;

use crate::mux::protocol::{ClientRequest, ServerResponse, VERSION, read_frame, write_frame};

use super::context::ServerContext;

pub(super) async fn handle(mut stream: TcpStream, context: Arc<ServerContext>) -> Result<()> {
    let request = tokio::time::timeout(Duration::from_secs(5), read_frame(&mut stream)).await??;
    let Some(ClientRequest::Authenticate { token, session }) = request else {
        bail!("authentication required");
    };
    if !crate::mux::token::matches(&token, &context.token) {
        write_frame(&mut stream, &error("authentication failed")).await?;
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
    while let Some(request) = read_frame(&mut stream).await? {
        let (response, close) = super::dispatch::apply(&context, session.as_deref(), request).await;
        write_frame(&mut stream, &response).await?;
        if matches!(response, ServerResponse::ShuttingDown) {
            context.shutdown.notify_one();
        }
        if close {
            break;
        }
    }
    Ok(())
}

fn error(message: &str) -> ServerResponse {
    ServerResponse::Error {
        message: message.into(),
    }
}
