//! Tests for session-picker row formatting.

use super::session_row_summary;
use crate::session::SessionSummary;
use chrono::Utc;
use std::path::PathBuf;

fn summary(title: Option<&str>, dir: Option<&str>) -> SessionSummary {
    SessionSummary {
        id: "abcdef1234567890".to_string(),
        title: title.map(str::to_string),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        message_count: 7,
        agent: "default".to_string(),
        directory: dir.map(PathBuf::from),
    }
}

#[test]
fn row_shows_id_prefix_title_dir_and_count() {
    let row = session_row_summary(&summary(Some("Fix build"), Some("/home/r/proj")), false);
    assert!(row.starts_with("abcdef12"));
    assert!(row.contains("Fix build"));
    assert!(
        row.contains("proj"),
        "directory basename disambiguates: {row}"
    );
    assert!(row.contains("7 msgs"));
}

#[test]
fn same_title_different_dirs_render_distinctly() {
    let a = session_row_summary(&summary(Some("fix tests"), Some("/a/alpha")), false);
    let b = session_row_summary(&summary(Some("fix tests"), Some("/b/beta")), false);
    assert_ne!(a, b, "same title in different dirs must differ");
    assert!(a.contains("alpha") && b.contains("beta"));
}

#[test]
fn active_marker_and_untitled_default() {
    let row = session_row_summary(&summary(None, None), true);
    assert!(row.contains("Untitled session"));
    assert!(row.contains('●'), "active session shows marker: {row}");
}
