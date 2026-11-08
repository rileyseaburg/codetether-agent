//! Secret-bearing credential record and local expiry/shape validation.
use super::validation::{valid_secret, validate_scope};
use anyhow::{Result, anyhow, ensure};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Deliberately no Debug implementation: this record contains credentials.
#[derive(Serialize, Deserialize)]
pub(crate) struct Credentials {
    pub(crate) client_id: String,
    pub(crate) access_token: String,
    pub(crate) refresh_token: Option<String>,
    pub(crate) expires_at: u64,
    pub(crate) scope: String,
}

pub(super) fn now() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| anyhow!("System clock is invalid"))?
        .as_secs())
}

impl Credentials {
    pub(crate) fn fresh(&self) -> Result<bool> {
        Ok(self.expires_at.saturating_sub(60) > now()?)
    }
    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            valid_secret(&self.client_id)
                && valid_secret(&self.access_token)
                && self.refresh_token.as_deref().is_none_or(valid_secret),
            "Invalid Monarch credentials"
        );
        validate_scope(&self.scope, "mcp:read mcp:write")
    }
}
