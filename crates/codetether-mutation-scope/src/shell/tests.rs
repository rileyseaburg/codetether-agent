//! Regression tests for read-only commands previously blocked by leases.

#[test]
fn transcript_status_and_help_commands_are_read_only() {
    assert!(super::read_only(
        "systemctl --user status remote-employee-presence.service"
    ));
    assert!(super::read_only(
        "git -C /home/riley/remote-employee-standalone status --short"
    ));
    assert!(super::read_only("codetether run --help"));
}

#[test]
fn similarly_shaped_mutations_still_require_leases() {
    assert!(!super::read_only(
        "systemctl --user stop remote-employee-presence.service"
    ));
    assert!(!super::read_only("git -C /repo commit -m change"));
    assert!(!super::read_only("codetether run implement-feature"));
}
