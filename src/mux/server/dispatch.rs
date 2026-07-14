//! Authenticated mux request mutation and response mapping.

use std::sync::Arc;

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::context::ServerContext;

pub(super) async fn apply(
    context: &Arc<ServerContext>,
    request: ClientRequest,
) -> (ServerResponse, bool) {
    match request {
        ClientRequest::Authenticate { .. } => (error("already authenticated"), false),
        ClientRequest::Snapshot => (snapshot(context).await, false),
        ClientRequest::Detach => (ServerResponse::Detached, true),
        ClientRequest::Shutdown => (ServerResponse::ShuttingDown, true),
        request => match super::mutate::apply(context, request).await {
            Ok(()) => (snapshot(context).await, false),
            Err(message) => (error(&message), false),
        },
    }
}

async fn snapshot(context: &ServerContext) -> ServerResponse {
    ServerResponse::Snapshot {
        state: context.state.read().await.clone(),
    }
}

fn error(message: &str) -> ServerResponse {
    ServerResponse::Error {
        message: message.into(),
    }
}
