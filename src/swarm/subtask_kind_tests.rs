use super::{TaskKind, classify};

#[test]
fn schema_examples_have_non_mutating_intent() {
    assert_eq!(
        classify("Task 1", "Inspect the API", None),
        TaskKind::ReadOnly
    );
    assert_eq!(
        classify("Task 2", "Run focused tests", None),
        TaskKind::Verification
    );
}

#[test]
fn design_review_is_read_only() {
    assert_eq!(
        classify("chunk-design-review", "Design review", None),
        TaskKind::ReadOnly
    );
}

#[test]
fn explicit_mutation_wins_over_review_language() {
    assert_eq!(
        classify("reviewer", "Review and fix the API", Some("Researcher")),
        TaskKind::Mutating
    );
}

#[test]
fn read_only_context_does_not_trigger_mutation_substrings() {
    assert_eq!(
        classify("review", "Review existing implementation", None),
        TaskKind::ReadOnly
    );
    assert_eq!(
        classify("analysis", "Analyze build logs", None),
        TaskKind::ReadOnly
    );
    assert_eq!(
        classify("inspection", "Inspect removed code", None),
        TaskKind::ReadOnly
    );
}
