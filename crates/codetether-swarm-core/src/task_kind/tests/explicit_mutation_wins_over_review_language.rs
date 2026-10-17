//! Original explicit_mutation_wins_over_review_language test.
use super::*;

#[test]
fn explicit_mutation_wins_over_review_language() {
    assert_eq!(
        classify("reviewer", "Review and fix the API", Some("Researcher")),
        TaskKind::Mutating
    );
}
