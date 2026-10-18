use super::fixture::rlm_result;
use crate::try_bounded_summary;

#[test]
fn rejects_empty_body_even_when_successful() {
    let r = rlm_result(true, "   \n\n   ", 10_000, 80);
    assert!(try_bounded_summary(&r, 512).is_none());
}
