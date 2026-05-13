use super::prompt::append_guardrails;

#[test]
fn injects_validation_level_terms() {
    let prompt = append_guardrails("base".to_string());
    assert!(prompt.contains("mocked local"));
    assert!(prompt.contains("live deployment/Argo"));
    assert!(prompt.contains("failed live validation is not completion"));
    assert!(prompt.contains("all-encompassing task"));
    assert!(prompt.contains("TetherScript"));
    assert!(prompt.contains("Memory trapdoor"));
    assert!(prompt.contains("core-memory"));
    assert!(prompt.contains("Core memory protocol"));
    assert!(prompt.contains("Belief-guided recall"));
}

#[test]
fn injection_is_idempotent() {
    let once = append_guardrails("base".to_string());
    let twice = append_guardrails(once.clone());
    assert_eq!(once, twice);
}
