//! End-to-end reviewer runs against the scripted provider.

use crate::provider::ProviderRegistry;
use crate::review::{ReviewOutcome, ReviewSubject, review};

use super::fixture::{config, registry};

#[tokio::test]
async fn review_runs_the_loop_and_returns_the_scripted_verdict() {
    let registry = registry(r#"{"outcome":"approve","reason":"Serves the goal.","findings":[]}"#);
    let subject = ReviewSubject {
        tool: "apply_patch".into(),
        resource: "src/x.rs".into(),
        goal: Some("## Goal Governance\nOBJECTIVE: ship x".into()),
        ..Default::default()
    };
    let verdict = review(&config(), &registry, None, std::env::temp_dir(), subject).await;
    assert_eq!(verdict.outcome, ReviewOutcome::Approve);
    assert_eq!(verdict.reason, "Serves the goal.");
}

#[tokio::test]
async fn missing_model_escalates_instead_of_erroring() {
    let registry = ProviderRegistry::new();
    let mut config = config();
    config.model = None;
    let verdict = review(
        &config,
        &registry,
        None,
        std::env::temp_dir(),
        Default::default(),
    )
    .await;
    assert_eq!(verdict.outcome, ReviewOutcome::Escalate);
}
