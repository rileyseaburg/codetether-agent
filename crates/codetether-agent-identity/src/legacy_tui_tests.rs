//! Existing TUI regression cases moved beside their implementation.
use crate::{agent_avatar, agent_profile, format_agent_identity, format_relay_handoff_line};

#[test]
fn agent_avatar_is_stable_and_ascii() {
    let avatar = agent_avatar("planner");
    assert_eq!(avatar, agent_avatar("planner"));
    assert!(avatar.is_ascii());
    assert!(avatar.starts_with('[') && avatar.ends_with(']'));
}

#[test]
fn relay_handoff_line_shows_avatar_labels() {
    let line = format_relay_handoff_line("relay-1", 2, "planner", "coder");
    assert!(line.contains("relay relay-1"));
    assert!(line.contains("@planner"));
    assert!(line.contains("@coder"));
    assert!(line.contains('['));
}

#[test]
fn relay_handoff_line_formats_user_sender() {
    let line = format_relay_handoff_line("relay-2", 1, "user", "planner");
    assert!(line.contains("[you]"));
    assert!(line.contains("@planner"));
}

#[test]
fn planner_profile_has_expected_personality() {
    let profile = agent_profile("auto-planner");
    assert_eq!(profile.codename, "Strategist");
    assert!(profile.profile.contains("decomposition"));
}

#[test]
fn formatted_identity_includes_codename() {
    let identity = format_agent_identity("auto-coder");
    assert!(identity.contains("@auto-coder"));
    assert!(identity.contains("‹Forge›"));
}
