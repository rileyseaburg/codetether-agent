//! Existing auto-chat parsing and policy regression.
use super::*;

#[test]
fn ensure_required_relay_capabilities_adds_missing_caps() {
    let mut caps = vec!["planning".to_string(), "relay".to_string()];
    ensure_required_relay_capabilities(&mut caps);
    assert!(caps.iter().any(|cap| cap == "context-handoff"));
    assert!(caps.iter().any(|cap| cap == "rlm-aware"));
    assert!(caps.iter().any(|cap| cap == "autochat"));
}
