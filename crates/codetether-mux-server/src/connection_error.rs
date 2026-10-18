//! Authentication failure response construction.
use codetether_mux_protocol::ServerResponse;

pub(super) fn error(message: &str) -> ServerResponse {
    ServerResponse::Error {
        message: message.into(),
    }
}
