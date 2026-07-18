use super::{matches, resolve};

#[test]
fn resolves_relative_prefixes_from_the_current_agent() {
    assert_eq!(
        resolve("/root/researcher", Some("worker")).unwrap(),
        Some("/root/researcher/worker".into())
    );
    assert!(matches(
        "/root/researcher/worker/child",
        Some("/root/researcher/worker")
    ));
    assert!(!matches(
        "/root/researcher/worker2",
        Some("/root/researcher/worker")
    ));
}

#[test]
fn rejects_malformed_prefixes() {
    assert!(resolve("/root", Some("")).is_err());
    assert!(resolve("/root", Some("worker/")).is_err());
    assert!(resolve("/root", Some("/other/worker")).is_err());
}
