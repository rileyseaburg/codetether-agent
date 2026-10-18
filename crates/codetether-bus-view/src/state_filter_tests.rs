//! Filter behavior tests for the bus log state.

use super::{BusLogState, test_support::entry};

#[test]
fn bus_filter_mode_can_be_entered_and_exited() {
    let mut state = BusLogState::new();
    assert!(!state.filter_input_mode);
    state.enter_filter_mode();
    assert!(state.filter_input_mode);
    state.exit_filter_mode();
    assert!(!state.filter_input_mode);
}

#[test]
fn bus_filter_chars_update_visible_entries() {
    let mut state = BusLogState::new();
    state.push(entry("alpha event", "protocol.alpha"));
    state.push(entry("beta event", "protocol.beta"));

    assert_eq!(state.visible_count(), 2);
    state.push_filter_char('b');
    state.push_filter_char('e');

    assert_eq!(state.filter, "be");
    assert_eq!(state.visible_count(), 1);
    assert_eq!(
        state.selected_entry().map(|entry| entry.summary.as_str()),
        Some("beta event")
    );
}

#[test]
fn bus_filter_backspace_and_clear_restore_entries() {
    let mut state = BusLogState::new();
    state.push(entry("alpha event", "protocol.alpha"));
    state.push(entry("beta event", "protocol.beta"));

    state.push_filter_char('a');
    state.push_filter_char('l');
    assert_eq!(state.visible_count(), 1);

    state.pop_filter_char();
    assert_eq!(state.filter, "a");
    assert_eq!(state.visible_count(), 2);

    state.clear_filter();
    assert!(state.filter.is_empty());
    assert_eq!(state.visible_count(), 2);
}
