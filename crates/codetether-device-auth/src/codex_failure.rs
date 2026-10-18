//! Terminal error mapping for Codex device authorization polling.
use crate::{
    CodexDeviceCodeTokenResponse, codex_types::CodexDeviceErrorResponse, error_text::truncate_body,
};
use anyhow::Result;

pub(super) fn reject(
    status: reqwest::StatusCode,
    body: &str,
) -> Result<CodexDeviceCodeTokenResponse> {
    if let Ok(payload) = serde_json::from_str::<CodexDeviceErrorResponse>(&body)
        && let Some(error) = payload.error.as_deref()
    {
        let description = payload
            .error_description
            .as_deref()
            .unwrap_or("No error description provided");
        anyhow::bail!("Codex device authorization failed: {error} ({description})");
    }

    anyhow::bail!(
        "Codex device authorization failed ({}): {}",
        status,
        truncate_body(&body)
    );
}
