//! Application forage regression coverage.
use super::*;

#[test]
fn collect_opportunities_skips_complete_or_cancelled_work() {
    let mut okr = Okr::new("Ship growth loop", "Increase retained users");
    okr.status = OkrStatus::Cancelled;
    let mut kr = KeyResult::new(okr.id, "Retained users", 100.0, "%");
    kr.update_progress(10.0);
    okr.add_key_result(kr);

    let items = collect_opportunities(&[okr]);
    assert!(items.is_empty());
}
