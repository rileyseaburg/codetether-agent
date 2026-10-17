use super::*;

/// Encode remote configuration as standard Base64 containing JSON.
///
/// # Arguments
/// * `payload` - Task configuration to encode.
///
/// # Returns
/// The encoded payload.
///
/// # Errors
/// Returns an error if serialization fails.
///
/// # Examples
/// See [RemoteSubtaskPayload] for a round-trip example.
pub fn encode_payload(payload: &RemoteSubtaskPayload) -> Result<String> {
    let json = serde_json::to_vec(payload)?;
    Ok(STANDARD.encode(json))
}

/// Decode a standard Base64/JSON task payload.
///
/// # Arguments
/// * `payload_b64` - Encoded configuration.
///
/// # Returns
/// The decoded remote task.
///
/// # Errors
/// Returns an error for invalid Base64 or invalid payload JSON.
///
/// # Examples
/// See [RemoteSubtaskPayload] for a round-trip example.
pub fn decode_payload(payload_b64: &str) -> Result<RemoteSubtaskPayload> {
    let bytes = STANDARD
        .decode(payload_b64.as_bytes())
        .map_err(|e| anyhow!("Invalid base64 payload: {e}"))?;
    let payload = serde_json::from_slice::<RemoteSubtaskPayload>(&bytes)
        .map_err(|e| anyhow!("Invalid remote payload JSON: {e}"))?;
    Ok(payload)
}
