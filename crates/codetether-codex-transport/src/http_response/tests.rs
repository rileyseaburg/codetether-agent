use super::{annotate_retry_after, retry_after_seconds};

#[test]
fn preserves_delta_seconds_without_changing_other_errors() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::RETRY_AFTER, "12".parse().unwrap());
    assert_eq!(retry_after_seconds(&headers), Some(12));
    assert!(annotate_retry_after("busy".into(), Some(12)).contains("retry_after_seconds=12"));
    assert_eq!(
        annotate_retry_after("bad request".into(), None),
        "bad request"
    );
}
