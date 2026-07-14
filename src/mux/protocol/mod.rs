//! Versioned JSON-lines protocol used by mux servers and clients.

mod frame;
mod request;
mod response;

pub(super) use frame::{read_frame, write_frame};
pub(super) use request::ClientRequest;
pub(super) use response::ServerResponse;

pub(super) const VERSION: u16 = 1;
