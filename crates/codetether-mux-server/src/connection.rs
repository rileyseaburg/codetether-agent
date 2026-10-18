//! Authentication and request loop for one mux client.

use std::sync::Arc;

use anyhow::Result;
use tokio::net::TcpStream;

use codetether_mux_protocol::{ServerResponse, read_frame, write_frame};

use super::context::ServerContext;

pub(super) async fn handle(mut stream: TcpStream, context: Arc<ServerContext>) -> Result<()> {
    let session = super::authentication::accept(&mut stream, &context).await?;
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
