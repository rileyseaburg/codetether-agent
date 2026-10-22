mod support;
use codetether_context_policy::{
    derivation::DerivePolicy,
    pages::PageKind,
    state_header::{ConstraintCorrection, ConstraintRecord},
};
use codetether_provider_core::ContentPart;
use std::sync::Arc;
use support::{NoCalls, Session};

#[tokio::test]
async fn partial_corrections_survive_actual_incremental_and_forced_compaction_requests() {
    let mut session = Session::new();
    let old = ConstraintRecord::from_message(
        0,
        &support::message("Preserve evidence. Use red. Never deploy.".into()),
        PageKind::Constraint,
    )
    .unwrap();
    let new = ConstraintRecord::from_message(
        2,
        &support::message("Use blue.".into()),
        PageKind::Constraint,
    )
    .unwrap();
    let start = old.text.find("Use red.").unwrap();
    session.corrections.push(ConstraintCorrection {
        source_turn: 0,
        source_hash: old.text_hash.clone(),
        start,
        end: start + 8,
        replacement_turn: 2,
        replacement_hash: new.text_hash.clone(),
    });
    session.pins = vec![old, new];
    for force_keep_last in [None, Some(1), Some(2)] {
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
            force_keep_last,
            support::hooks(),
        )
        .await
        .unwrap();
        let ContentPart::Text { text } = &derived.messages[0].content[0] else {
            panic!()
        };
        assert!(text.contains("Preserve evidence."));
        assert!(text.contains("Never deploy."));
        assert!(text.contains("Use blue."));
        assert!(!text.contains("Use red."));
        assert_eq!(session.corrections.len(), 1);
    }
}
