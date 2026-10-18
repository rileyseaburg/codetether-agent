//! Help rows cover the current view registry and stable panel labels.
use crate::view_mode_help::view_mode_help_rows;

#[test]
fn view_mode_help_rows_include_every_view_mode() {
    let rows = view_mode_help_rows();
    assert_eq!(rows.len(), crate::view_mode_registry::ALL_VIEW_MODES.len());
    assert!(rows.iter().any(|row| row.contains("Chat")));
    assert!(rows.iter().any(|row| row.contains("Swarm")));
    assert!(rows.iter().any(|row| row.contains("Ralph")));
    assert!(rows.iter().any(|row| row.contains("Bus Log")));
    assert!(rows.iter().any(|row| row.contains("Protocol")));
    assert!(rows.iter().any(|row| row.contains("Sessions")));
    assert!(rows.iter().any(|row| row.contains("Model Picker")));
    assert!(rows.iter().any(|row| row.contains("Subagents")));
    assert!(rows.iter().any(|row| row.contains("File Picker")));
}
