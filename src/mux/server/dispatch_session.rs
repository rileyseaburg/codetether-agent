//! Requests that act on the session bound to the calling connection.

use std::sync::Arc;

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::context::ServerContext;

pub(super) async fn apply(
    context: &Arc<ServerContext>,
    session: &str,
    request: ClientRequest,
) -> ServerResponse {
    match request {
        ClientRequest::Agent { request } => super::agent::apply(context, session, request).await,
        ClientRequest::ReportRuntime { status } => {
            super::runtime::apply(context, session, status).await
        }
        ClientRequest::Program { request } => {
            super::program_request::execute(context, session, request)
                .await
                .unwrap_or_else(|error| super::dispatch::error(&error.to_string()))
        }
        request => match super::mutate::apply(context, session, request).await {
            Ok(()) => super::dispatch::snapshot(context).await,
            Err(message) => super::dispatch::error(&message),
        },
    }
}
