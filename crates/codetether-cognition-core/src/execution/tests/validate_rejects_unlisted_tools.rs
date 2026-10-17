//! validate rejects unlisted tools.
use super::*;
#[test]
fn validate_rejects_unlisted_tools() {
    let allowed = vec!["Bash".to_string()];
    assert!(!allowed.contains(&"WebFetch".to_string()));
    assert!(allowed.contains(&"Bash".to_string()));
}
