//! Existing objective outcome and evidence regression assertions.

use crate::outcomes::map_stories_to_kr_outcomes;

#[test]
fn map_stories_creates_outcomes_for_each_kr() {
    let (mut okr, mut run) = super::objective::create();
    let prd = super::prd::create();
    let state = super::state::create(&prd);
    map_stories_to_kr_outcomes(&mut okr, &mut run, &prd, &state);

    // Should have 2 outcomes (one per KR)
    assert_eq!(run.outcomes.len(), 2);
    // Each outcome should reference the correct KR
    assert_eq!(run.outcomes[0].kr_id, okr.key_results[0].id);
    assert_eq!(run.outcomes[1].kr_id, okr.key_results[1].id);
    // Progress should be 50% (1/2 stories passed)
    assert_eq!(run.outcomes[0].value, Some(50.0)); // 0.5 * 100.0
    // Evidence should include story results
    assert!(
        run.outcomes[0]
            .evidence
            .iter()
            .any(|e| e.contains("US-001"))
    );
    assert!(
        run.outcomes[0]
            .evidence
            .iter()
            .any(|e| e.contains("PASSED"))
    );
    assert!(
        run.outcomes[0]
            .evidence
            .iter()
            .any(|e| e.contains("FAILED"))
    );
}
