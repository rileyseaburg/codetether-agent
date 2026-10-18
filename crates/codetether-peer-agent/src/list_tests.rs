use super::local;

#[test]
fn marks_the_active_process_as_self() {
    let entry = local(Some("workspace-agent-ab12".into())).unwrap();

    assert_eq!(entry["name"], "workspace-agent-ab12");
    assert_eq!(entry["self"], true);
    assert_eq!(entry["kind"], "lan-peer");
}

#[test]
fn omits_self_when_no_endpoint_is_active() {
    assert!(local(None).is_none());
}
