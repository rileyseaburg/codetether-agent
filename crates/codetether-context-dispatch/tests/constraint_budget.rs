mod support;
use codetether_context_policy::derivation::DerivePolicy;
use codetether_provider_core::ContentPart;
use codetether_request_tokens::estimate_request_tokens;
use std::sync::Arc;
use support::{NoCalls, Session};

#[tokio::test]
async fn complete_persistent_constraints_are_reserved_before_selecting_history() {
    let session = Session::new();
    let original = serde_json::to_string(&session.messages).unwrap();
    let derived = codetether_context_dispatch::derive_with_policy::<_, ()>(
        &session,
        Arc::new(NoCalls),
        "gpt-4o",
        "fixed policy",
        &[],
        None,
        DerivePolicy::Incremental {
            budget_tokens: 5000,
        },
        None,
        support::hooks(),
    )
    .await
    .unwrap();
    assert!(estimate_request_tokens("fixed policy", &derived.messages, &[]) <= 5000);
    let ContentPart::Text { text } = &derived.messages[0].content[0] else {
        unreachable!()
    };
    assert!(text.contains(&session.pins[0].text));
    assert!(
        derived
            .provenance
            .iter()
            .any(|tag| tag == "constraint_sources")
    );
    assert_eq!(derived.messages.len(), derived.resolutions.len());
    assert_eq!(serde_json::to_string(&session.messages).unwrap(), original);
}

#[tokio::test]
async fn impossible_pin_budget_is_an_explicit_error_instead_of_silent_loss() {
    let session = Session::new();
    let result = codetether_context_dispatch::derive_with_policy::<_, ()>(
        &session,
        Arc::new(NoCalls),
        "gpt-4o",
        "fixed policy",
        &[],
        None,
        DerivePolicy::Incremental { budget_tokens: 100 },
        None,
        support::hooks(),
    )
    .await;
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("pinned source text alone")
    );
}
