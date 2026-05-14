use super::prompt::append_guardrails_for_cwd;
use std::path::Path;

#[test]
fn injects_validation_level_terms() {
    let prompt = append_guardrails_for_cwd("base".to_string(), Path::new("."));
    assert!(prompt.contains("mocked local"));
    assert!(prompt.contains("live deployment/Argo"));
    assert!(prompt.contains("failed live validation is not completion"));
    assert!(prompt.contains("all-encompassing task"));
    assert!(prompt.contains("TetherScript"));
    assert!(prompt.contains("Memory trapdoor"));
    assert!(prompt.contains("core-memory"));
    assert!(prompt.contains("Core memory protocol"));
    assert!(prompt.contains("Belief-guided recall"));
    assert!(prompt.contains("Workflow evidence templates"));
    assert!(prompt.contains("Runtime prefetch facts"));
}

#[test]
fn injection_is_idempotent() {
    let once = append_guardrails_for_cwd("base".to_string(), Path::new("."));
    let twice = append_guardrails_for_cwd(once.clone(), Path::new("."));
    assert_eq!(once, twice);
}
