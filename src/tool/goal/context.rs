//! Trusted session identity extraction for goal tool calls.

use anyhow::{Result, anyhow};

pub(super) fn session_id(injected: Option<String>) -> Result<String> {
    injected
        .or_else(|| std::env::var("CODETETHER_SESSION_ID").ok())
        .ok_or_else(|| anyhow!("session id not available; cannot locate goal"))
}
