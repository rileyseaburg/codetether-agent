//! Recognize generated metadata without consuming caller-owned lookalikes.

/// Recognize only the canonical metadata emitted by this harness.
pub(super) fn is_identity(text: &str) -> bool {
    identity_len(text) == Some(text.len())
}

/// Locate canonical metadata using parsed JSON, never marker text in its values.
pub(super) fn identity_len(text: &str) -> Option<usize> {
    let body = text.strip_prefix(super::START).and_then(|body| {
        body.strip_prefix(
            "Authoritative routing identity supplied by the CodeTether harness for this request: ",
        )
    })?;
    let identity = serde_json::Deserializer::from_str(body)
        .into_iter::<serde_json::Value>()
        .next()?
        .ok()?;
    let canonical = super::prompt(identity["provider"].as_str()?, identity["model"].as_str()?);
    text.starts_with(&canonical).then_some(canonical.len())
}
