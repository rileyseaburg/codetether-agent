//! Authentication client_id responsibilities.
use anyhow::Result;

pub(super) fn resolve_client_id(client_id: Option<String>) -> Result<String> {
    let id = client_id
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "GitHub OAuth client ID is required. Pass `--client-id <id>` or set `CODETETHER_COPILOT_OAUTH_CLIENT_ID`."
            )
        })?;

    Ok(id)
}
