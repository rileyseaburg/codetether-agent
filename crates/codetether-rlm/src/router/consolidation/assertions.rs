//! Expected loop exit outcomes.
pub(super) fn check(results: &[serde_json::Value]) {
    assert_eq!(results[0]["answer"], "done");
    assert_eq!(results[0]["subcalls"], 1);
    assert_eq!(results[1]["iterations"], 1);
    assert_eq!(results[2]["iterations"], 2);
    assert_eq!(results[3]["subcalls"], 1);
    assert_eq!(results[4]["subcalls"], 2);
    assert_eq!(results[5]["aborted"], true);
    assert_eq!(results[7]["answer"], "tool final");
}
