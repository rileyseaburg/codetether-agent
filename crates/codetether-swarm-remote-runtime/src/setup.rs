//! Payload loading and remote workspace selection.
use anyhow::{Context, Result};
use codetether_swarm_core::remote_protocol::{RemoteSubtaskPayload, decode_payload};
use std::path::PathBuf;
pub(super) fn payload(
    payload_base64: Option<String>,
    payload_env: String,
) -> Result<RemoteSubtaskPayload> {
    let payload_b64 = if let Some(payload) = payload_base64 {
        payload
    } else {
        std::env::var(&payload_env)
            .with_context(|| format!("Missing swarm payload env var {}", payload_env))?
    };
    let payload = decode_payload(&payload_b64)?;

    Ok(payload)
}
pub(super) fn workspace(payload: &RemoteSubtaskPayload) -> Result<PathBuf> {
    let working_dir = payload
        .working_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    std::env::set_current_dir(&working_dir)
        .with_context(|| format!("Failed to set working dir to {}", working_dir.display()))?;

    Ok(working_dir)
}
