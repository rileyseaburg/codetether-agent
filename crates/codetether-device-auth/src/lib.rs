//! Codex and GitHub device-authorization requests and polling.
//!
//! Call a request function to obtain a device code, then its polling function
//! to wait for authorization. The caller supplies HTTP clients, endpoint
//! identity and client identity; token exchange/storage remain with the caller.
mod codex_poll;
mod codex_request;
mod codex_types;
mod error_text;
mod github_poll;
mod github_request;
mod github_types;
mod interval;
mod timing;
pub use codex_poll::poll_for_codex_authorization_code;
pub use codex_request::request_codex_device_code;
pub use codex_types::{CodexDeviceCodeResponse, CodexDeviceCodeTokenResponse};
pub use github_poll::poll_for_access_token;
pub use github_request::request_device_code;
pub use github_types::DeviceCodeResponse;

const OAUTH_POLLING_SAFETY_MARGIN_MS: u64 = 3000;

const CODEX_DEVICE_AUTH_TIMEOUT_SECS: u64 = 15 * 60;

#[cfg(test)]
mod tests;

mod codex_failure;
mod codex_poll_request;
mod codex_start_request;
mod github_poll_request;
