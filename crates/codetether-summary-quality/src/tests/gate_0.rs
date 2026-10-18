use super::fixture::rlm_result;
use crate::try_bounded_summary;

#[test]
fn rejects_unsuccessful_results() {
    let r = rlm_result(false, "ignored", 1024, 256);
    assert!(try_bounded_summary(&r, 512).is_none());
}
