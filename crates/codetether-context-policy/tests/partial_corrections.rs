use codetether_context_policy::{
    pages::PageKind,
    state_header::{ConstraintCorrection, ConstraintRecord, prepend_state_header_with_corrections},
};
use codetether_provider_core::{ContentPart, Message, Role};

fn source(turn: usize, text: &str) -> ConstraintRecord {
    ConstraintRecord::from_message(
        turn,
        &Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: text.into() }],
        },
        PageKind::Constraint,
    )
    .unwrap()
}
fn edit(old: &ConstraintRecord, text: &str, next: &ConstraintRecord) -> ConstraintCorrection {
    let start = old.text.find(text).unwrap();
    ConstraintCorrection {
        source_turn: old.source_turn,
        source_hash: old.text_hash.clone(),
        start,
        end: start + text.len(),
        replacement_turn: next.source_turn,
        replacement_hash: next.text_hash.clone(),
    }
}
fn header(records: Vec<ConstraintRecord>, edits: &[ConstraintCorrection]) -> String {
    let mut messages = vec![];
    prepend_state_header_with_corrections(records, &[], &[], || 10_000, edits, &mut messages);
    match &messages[0].content[0] {
        ContentPart::Text { text } => text.clone(),
        _ => panic!(),
    }
}

#[test]
fn disjoint_unicode_corrections_preserve_all_unrelated_requirements_after_serialization() {
    let old = source(
        0,
        "Never delete evidence. Use 🦀 red. Test weekly. Do not deploy.",
    );
    let blue = source(3, "Use 🦀 blue instead.");
    let daily = source(5, "Test daily.");
    let edits = vec![edit(&old, "🦀 red", &blue), edit(&old, "weekly", &daily)];
    let edits =
        serde_json::from_str::<Vec<ConstraintCorrection>>(&serde_json::to_string(&edits).unwrap())
            .unwrap();
    let rendered = header(vec![old.clone(), blue, daily], &edits);
    assert!(rendered.contains("Never delete evidence."));
    assert!(rendered.contains("All unreplaced user requirements remain active"));
    assert!(rendered.contains("Do not deploy."));
    assert!(rendered.contains("🦀 blue"));
    assert!(rendered.contains("Test daily."));
    assert!(!rendered.contains("🦀 red"));
    assert!(!rendered.contains("weekly"));
    assert!(
        old.text.contains("🦀 red"),
        "durable source text is untouched"
    );
}

#[test]
fn stale_or_missing_sources_never_suppress_original_constraints() {
    let old = source(0, "Use red. Preserve evidence.");
    let next = source(1, "Use blue.");
    let link = edit(&old, "Use red.", &next);
    for records in [
        vec![old.clone()],
        vec![old.clone(), source(1, "changed source")],
    ] {
        let rendered = header(records, &[link.clone()]);
        assert!(rendered.contains("CONSTRAINT_CORRECTION_UNRESOLVED"));
        assert!(rendered.contains("Use red. Preserve evidence."));
    }
}

#[test]
fn invalid_unicode_and_overlapping_spans_are_conservative() {
    let old = source(0, "🦀 red. Preserve evidence.");
    let next = source(2, "Use blue.");
    let mut invalid = edit(&old, "🦀 red", &next);
    invalid.start = 1;
    assert!(
        header(vec![old.clone(), next.clone()], &[invalid]).contains("🦀 red. Preserve evidence.")
    );
    let a = edit(&old, "🦀 red", &next);
    let b = edit(&old, "red.", &next);
    assert!(header(vec![old, next], &[a, b]).contains("CONSTRAINT_CORRECTION_UNRESOLVED"));
}

#[test]
fn partial_then_whole_replacement_follows_the_current_user_source() {
    let old = source(0, "Use red. Keep evidence.");
    let mut next = source(1, "Use blue.");
    let final_source = source(3, "Use green.");
    let link = edit(&old, "Use red.", &next);
    next.superseded_by = Some(3);
    let rendered = header(vec![old, next, final_source], &[link]);
    assert!(rendered.contains("by user source turn 3"));
    assert!(rendered.contains("Use green."));
    assert!(rendered.contains("Keep evidence."));
    assert!(!rendered.contains("Use red."));
    assert!(!rendered.contains("Use blue."));
}
