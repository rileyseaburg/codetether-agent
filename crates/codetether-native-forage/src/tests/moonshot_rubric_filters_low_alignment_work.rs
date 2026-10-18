//! Application forage regression coverage.
use super::*;

#[test]
fn moonshot_rubric_filters_low_alignment_work() {
    let mut okr = Okr::new(
        "Improve parser latency",
        "Reduce p95 latency for parser pipeline",
    );
    okr.status = OkrStatus::Active;
    let mut kr = KeyResult::new(okr.id, "Parser p95 under 50ms", 100.0, "%");
    kr.update_progress(10.0);
    okr.add_key_result(kr);

    let rubric = MoonshotRubric {
        goals: vec!["eliminate billing fraud globally".to_string()],
        required: true,
        min_alignment: 0.4,
    };

    let items = collect_opportunities_with_rubric(&[okr], &rubric);
    assert!(
        items.is_empty(),
        "non-aligned work should be filtered out when moonshot is required"
    );
}
