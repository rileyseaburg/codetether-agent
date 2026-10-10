//! Authenticated companion relay transport with memory-only credentials.
mod client;
mod commands;
mod replies;
pub(crate) use commands::{Commands, Reply};
mod device;
mod error;
mod frame_input;
mod http;
mod pair;
mod pause;
mod poll;
mod upload;
mod upload_body;
mod validation;
mod wire;

pub(crate) use device::Device;
pub(crate) use error::Error;

/// Normalizes user input the same way the relay does: 12 hex characters.
pub(crate) fn normalize(code: &str) -> Option<String> {
    let code: String = code
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_ascii_uppercase();
    (code.len() == 12 && code.chars().all(|c| c.is_ascii_hexdigit())).then_some(code)
}

/// Exchanges a one-use code for a session device token.
/// Errors are user-facing and never contain the code or token.
pub(crate) fn pair(code: &str) -> Result<Device, String> {
    pair::exchange(code).map_err(Error::pair_message)
}
