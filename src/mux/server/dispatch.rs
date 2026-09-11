//! Authenticated mux request routing by scope.
//!
//! Server-scoped requests (snapshot, session lifecycle, coordination,
//! shutdown) run on any connection. Session-scoped requests require the
//! connection to have bound a session at authentication.

use std::sync::Arc;

use crate::mux::protocol::{ClientRequest, ServerResponse};

use super::context::ServerContext;

pub(super) async fn apply(
    context: &Arc<ServerContext>,
    session: Option<&str>,
    request: ClientRequest,
) -> (ServerResponse, bool) {
    match request {
        ClientRequest::Authenticate { .. } => (error("already authenticated"), false),
        ClientRequest::Snapshot => (snapshot(context).await, false),
        ClientRequest::Detach => (ServerResponse::Detached, true),
        ClientRequest::Shutdown => (ServerResponse::ShuttingDown, true),
        ClientRequest::Coordinate { request } => {
            (super::coordination::apply(context, request).await, false)
        }
        ClientRequest::CreateSession { name, workspace } => (
            super::session_create::create(context, name, workspace).await,
            false,
        ),
        ClientRequest::CloseSession { name } => super::session_close::close(context, &name).await,
        request => {
            let Some(session) = session else {
                return (error("request requires a bound mux session"), false);
            };
            (
                super::dispatch_session::apply(context, session, request).await,
                false,
            )
        }
    }
}

pub(super) async fn snapshot(context: &ServerContext) -> ServerResponse {
    ServerResponse::Snapshot {
        state: context.state.read().await.clone(),
    }
}

pub(super) fn error(message: &str) -> ServerResponse {
    ServerResponse::Error {
        message: message.into(),
    }
}
