//! Server dispatch for persistent PTY program operations.

use std::sync::Arc;

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::context::ServerContext;

pub(super) async fn apply(context: &Arc<ServerContext>, request: ClientRequest) -> ServerResponse {
    let ClientRequest::Program { request } = request else {
        unreachable!("program dispatcher received control request")
    };
    let result = super::program_request::execute(context, request).await;
    result.unwrap_or_else(|error| ServerResponse::Error {
        message: error.to_string(),
    })
}
