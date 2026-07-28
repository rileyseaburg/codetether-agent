//! Minimal blocking HTTP/1.1 server and client.
//!
//! This module re-exports the public API from focused sub-modules:
//!
//! - [`get`], [`head`], [`post`], [`request`] - blocking client functions
//! - [`serve`] - blocking server accept loop
//! - `http://` URL parsing used by the client and server helpers
//!
//! Internal sub-modules handle headers, response serialization,
//! response extraction, and status reason phrases.

#[path = "http_client.rs"]
mod http_client;
#[path = "http_headers.rs"]
mod http_headers;
#[path = "http_response.rs"]
mod http_response;
#[path = "http_response_extract.rs"]
mod http_response_extract;
#[path = "http_server.rs"]
mod http_server;
#[path = "http_static/mod.rs"]
mod http_static;
#[path = "http_status.rs"]
mod http_status;
#[path = "http_stream.rs"]
mod http_stream;
#[path = "http_url.rs"]
mod http_url;

pub(crate) use http_client::client_request;
pub use http_client::{get, head, post, request};
pub use http_server::serve;
pub(crate) use http_static::serve as serve_static;

#[cfg(test)]
#[path = "http_tests.rs"]
mod tests;
