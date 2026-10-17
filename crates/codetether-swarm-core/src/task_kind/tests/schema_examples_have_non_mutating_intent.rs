//! Original schema_examples_have_non_mutating_intent test.
use super::*;

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
