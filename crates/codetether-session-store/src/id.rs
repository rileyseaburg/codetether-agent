//! Validate IDs at every persistence boundary, including imported snapshots.
/// Reject invalid session identifiers at persistence boundaries.
/// # Arguments
/// `id` is the session identity being persisted.
/// # Returns
/// Unit for a nonempty identifier containing only permitted characters.
/// # Errors
/// The identifier is empty, too long, or contains forbidden characters.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::id::validate;
/// # fn example() -> anyhow::Result<()> {
/// let result = validate("session-123")?;
/// # Ok(()) }
/// ```
pub fn validate(id: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        !id.is_empty()
            && id.len() <= 128
            && id
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
        "Invalid session ID: rejecting path traversal risk"
    );
    Ok(())
}
#[test]
fn refuses_paths_and_empty_ids() {
    for id in ["", "../escape", "a/b", "a\\b"] {
        assert!(validate(id).is_err());
    }
    assert!(validate("stable-session-123").is_ok());
}
