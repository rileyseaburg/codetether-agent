use super::fixture::rlm_result;
use crate::try_bounded_summary;

#[test]
fn rejects_production_770k_to_180_case() {
    let r = rlm_result(true, "body irrelevant for the count", 770_000, 180);
    assert!(try_bounded_summary(&r, 512).is_none());
}
