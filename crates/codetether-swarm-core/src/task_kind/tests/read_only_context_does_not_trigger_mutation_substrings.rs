//! Original read_only_context_does_not_trigger_mutation_substrings test.
use super::*;

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
