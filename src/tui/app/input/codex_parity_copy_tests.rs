//! Copy command validation must not accidentally submit or retain commands.

use crate::tui::app::state::App;

#[test]
fn invalid_copy_target_clears_input_and_reports_usage() {
    let mut app = App::default();
    app.state.input = "/copy invalid".into();
    super::run(&mut app, "/copy invalid");
    assert!(app.state.input.is_empty());
    assert_eq!(app.state.status, "Usage: /copy [reply|tool|error]");
}

#[test]
fn missing_tool_output_clears_input_and_reports_no_match() {
    let mut app = App::default();
    app.state.input = "/copy tool".into();
    super::run(&mut app, "/copy tool");
    assert!(app.state.input.is_empty());
    assert_eq!(app.state.status, "No matching message to copy.");
}

#[test]
fn missing_error_does_not_copy_system_status() {
    let mut app = App::default();
    app.state.messages.clear();
    super::run(&mut app, "/copy error");
    assert!(app.state.input.is_empty());
    assert_eq!(app.state.status, "No matching message to copy.");
}
