//! Loopback OAuth callback capture and manual callback parsing.
//!
//! [capture_oauth_callback_auto] waits on the configured Codex localhost port,
//! with manual-paste fallback. [extract_oauth_code_and_state] parses a pasted
//! URL/query; [parse_oauth_callback_request] parses its HTTP request form.
mod capture;
mod extract;
mod query;
mod read;
mod request;
mod response;
mod wait;
mod wait_any;
pub use capture::capture_oauth_callback_auto;
pub use extract::extract_oauth_code_and_state;
pub use request::parse_oauth_callback_request;

const CODEX_CALLBACK_ADDR_V4: &str = "127.0.0.1:1455";

const CODEX_CALLBACK_ADDR_V6: &str = "[::1]:1455";

const CODEX_CALLBACK_DISPLAY_ADDR: &str = "localhost:1455";

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_http;

mod bind;
