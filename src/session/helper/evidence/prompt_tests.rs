use super::prompt::append_guardrails_for_cwd;
use std::path::Path;

#[test]
fn injects_validation_level_terms() {
    let prompt = append_guardrails_for_cwd("base".to_string(), Path::new("."), true);
    assert!(prompt.contains("mocked local"));
    assert!(prompt.contains("live deployment/Argo"));
    assert!(prompt.contains("failed live validation is not completion"));
    assert!(prompt.contains("do not block ordinary PR review on post-merge evidence"));
    assert!(prompt.contains("do not require post-merge Argo proof for review-only tasks"));
    assert!(prompt.contains("all-encompassing task"));
    assert!(prompt.contains("TetherScript"));
    assert!(prompt.contains("Memory trapdoor"));
    assert!(prompt.contains("core-memory"));
    assert!(prompt.contains("Core memory protocol"));
    assert!(prompt.contains("named source of truth"));
    assert!(prompt.contains("persists until explicitly revoked"));
    assert!(prompt.contains("Belief-guided recall"));
    assert!(prompt.contains("Workflow evidence templates"));
    assert!(prompt.contains("Runtime prefetch facts"));
}

#[test]
fn injection_is_idempotent() {
    let once = append_guardrails_for_cwd("base".to_string(), Path::new("."), true);
    let twice = append_guardrails_for_cwd(once.clone(), Path::new("."), true);
    assert_eq!(once, twice);
}

#[test]
fn skips_prior_context_when_user_disables_it() {
    let prompt = append_guardrails_for_cwd("base".to_string(), Path::new("."), false);
    assert!(prompt.contains("not loaded at the user's request"));
    assert!(!prompt.contains("Recent proven deliverables:"));
    assert!(!prompt.contains("Use memory.search"));
    assert!(!prompt.contains("Save durable decisions"));
}
