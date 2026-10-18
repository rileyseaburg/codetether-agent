use super::fixture::rlm_result;
use crate::try_bounded_summary;

#[test]
fn accepts_plausible_output() {
    let body = "a real-looking summary that exceeds the degraded floor".repeat(4);
    let r = rlm_result(true, &body, 10_000, 80);
    assert!(try_bounded_summary(&r, 512).is_some());
}
