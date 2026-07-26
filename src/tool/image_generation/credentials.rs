use super::auth::ImagesAuth;
use anyhow::{Result, bail};

pub(super) async fn resolve() -> Result<ImagesAuth> {
    if let Some(auth) = super::vault_credentials::resolve().await? {
        return Ok(auth);
    }
    bail!("image credentials unavailable in Vault provider `openai-codex`")
}