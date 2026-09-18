//! Typed device response and protocol outcomes.
use anyhow::{Result, bail};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Reply {
    error: Option<String>,
    id_token: Option<String>,
    access_token: Option<String>,
}

pub(super) enum Outcome {
    Pending,
    SlowDown,
    Token(String),
}
pub(super) fn interpret(reply: Reply, status: reqwest::StatusCode) -> Result<Outcome> {
    match reply.error.as_deref() {
        Some("authorization_pending") => Ok(Outcome::Pending),
        Some("slow_down") => Ok(Outcome::SlowDown),
        Some("access_denied") => bail!("Device login was declined"),
        Some("expired_token") => bail!("Device login expired"),
        Some(_) => bail!("Device login rejected; verify public-client configuration"),
        None if status.is_success() => reply
            .id_token
            .or(reply.access_token)
            .filter(|token| !token.is_empty())
            .map(Outcome::Token)
            .ok_or_else(|| anyhow::anyhow!("Device response omitted a token")),
        None => bail!("Device token request rejected (HTTP {})", status.as_u16()),
    }
}
