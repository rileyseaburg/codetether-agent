//! Structured diagnostics containing request shape and backend only.
mod log_http_responses_request;
mod log_responses_ws_request;
pub use log_http_responses_request::log_http_responses_request;
pub use log_responses_ws_request::log_responses_ws_request;
