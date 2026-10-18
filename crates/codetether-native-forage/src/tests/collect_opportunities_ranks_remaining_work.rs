//! Application forage regression coverage.
use super::*;

#[test]
fn collect_opportunities_ranks_remaining_work() {
    let mut okr = Okr::new("Ship growth loop", "Increase retained users");
    okr.status = OkrStatus::Active;

    let mut kr_low = KeyResult::new(okr.id, "KR Low Remaining", 100.0, "%");
    kr_low.update_progress(80.0);
    let mut kr_high = KeyResult::new(okr.id, "KR High Remaining", 100.0, "%");
    kr_high.update_progress(10.0);
    okr.add_key_result(kr_low);
    okr.add_key_result(kr_high);

    let items = collect_opportunities(&[okr]);
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].key_result_title, "KR High Remaining");
}
