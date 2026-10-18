use super::fixture::rlm_result;
use crate::try_bounded_summary;

#[test]
fn rejects_tiny_output_for_large_input() {
    let r = rlm_result(true, "summary", 770_000, 12);
    assert!(try_bounded_summary(&r, 512).is_none());
}
