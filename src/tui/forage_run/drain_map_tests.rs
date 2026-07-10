//! Tests for forage update presentation.

use super::*;

#[test]
fn scan_completion_says_that_nothing_executed() {
    let mut app = App::default();
    app.state.forage.active = true;

    let (message, status) = map_update(
        &mut app,
        ForageUpdate::ScanComplete("Found three opportunities.".to_string()),
    );

    assert!(message.contains("no work was executed"));
    assert!(message.contains("`/forage execute`"));
    assert_eq!(status, "Forage scan complete");
    assert!(!app.state.forage.active);
}
