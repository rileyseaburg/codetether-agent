//! Validate IDs at every persistence boundary, including imported snapshots.
pub(crate) fn validate(id: &str) -> anyhow::Result<()> {
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
