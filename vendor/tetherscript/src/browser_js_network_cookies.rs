//! Server-cookie state for native browser networking.

#[path = "browser_js_network_cookies_request.rs"]
mod request;
#[path = "browser_js_network_cookies_response.rs"]
mod response;
#[path = "browser_js_network_cookies_state.rs"]
mod state;

pub(crate) use request::append_request_header;
pub(crate) use response::{apply_document_cookie, apply_response_headers};
pub(crate) use state::{jar, reset, seed, set_document_url};
