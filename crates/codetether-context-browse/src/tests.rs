//! Parsing and path-listing regression coverage.
use crate::listing::render_listing;
use crate::*;
use serde_json::json;

use std::path::{Path, PathBuf};

#[test]
fn parse_browse_action_defaults_to_list() {
    let action = parse_browse_action(&json!({})).unwrap();
    assert!(matches!(action, ContextBrowseAction::ListTurns));
}

#[test]
fn parse_browse_action_requires_turn_for_show() {
    let err = parse_browse_action(&json!({"action": "show_turn"})).unwrap_err();
    assert!(err.contains("turn"));
}

#[test]
fn parse_browse_action_rejects_unknown() {
    let err = parse_browse_action(&json!({"action": "truncate"})).unwrap_err();
    assert!(err.contains("unknown"));
}

#[test]
fn format_turn_path_uses_real_filesystem_paths() {
    let path0 = format_turn_path(Path::new("/tmp/history/sid"), 0, "user");
    let path = format_turn_path(Path::new("/tmp/history/sid"), 7, "assistant");
    assert_eq!(path0, PathBuf::from("/tmp/history/sid/turn-0000-user.md"));
    assert_eq!(
        path,
        PathBuf::from("/tmp/history/sid/turn-0007-assistant.md")
    );
}

#[test]
fn render_listing_emits_one_path_per_turn() {
    let listing = render_listing(&[
        PathBuf::from("/tmp/history/sid/turn-0000-user.md"),
        PathBuf::from("/tmp/history/sid/turn-0001-assistant.md"),
    ]);
    assert_eq!(listing.lines().count(), 2);
    assert!(listing.contains("/tmp/history/sid/turn-0000-user.md"));
    assert!(listing.contains("/tmp/history/sid/turn-0001-assistant.md"));
}
