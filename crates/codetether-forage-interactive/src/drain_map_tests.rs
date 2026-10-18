//! Tests for forage update presentation.

use super::*;

#[test]
fn scan_completion_says_that_nothing_executed() {
    let mut state = ForageState::new();
    state.active = true;

    let (message, status) = map_update(
        &mut state,
        ForageUpdate::ScanComplete("Found three opportunities.".to_string()),
    );

    assert!(message.contains("no work was executed"));
    assert!(message.contains("`/forage execute`"));
    assert_eq!(status, "Forage scan complete");
    assert!(!state.active);
}
