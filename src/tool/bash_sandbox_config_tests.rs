use super::{from_mode, state};
use crate::config::SandboxMode;

#[test]
fn danger_full_access_disables_sandbox() {
    assert!(!from_mode(true, SandboxMode::DangerFullAccess));
}

#[test]
fn restricted_modes_keep_default_decision() {
    assert!(from_mode(true, SandboxMode::ReadOnly));
    assert!(from_mode(true, SandboxMode::WorkspaceWrite));
    assert!(!from_mode(false, SandboxMode::ReadOnly));
}

#[test]
fn read_only_commands_do_not_use_os_sandbox() {
    assert!(!state::enabled_for_read_class(true, "date"));
    assert!(!state::enabled_for_read_class(true, "git status"));
    assert!(state::enabled_for_read_class(true, "touch changed"));
}

#[test]
fn approved_fallback_disables_unavailable_sandbox() {
    assert!(!state::enabled_for_state(true, false, true, false, true));
    assert!(state::enabled_for_state(true, false, true, false, false));
    assert!(state::enabled_for_state(true, false, false, false, true));
}
