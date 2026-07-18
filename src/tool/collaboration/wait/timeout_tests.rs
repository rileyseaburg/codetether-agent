use super::duration;

#[test]
fn enforces_codex_v2_timeout_range() {
    assert!(duration(9_999).is_err());
    assert_eq!(duration(30_000).unwrap().as_millis(), 30_000);
    assert!(duration(3_600_001).is_err());
}
