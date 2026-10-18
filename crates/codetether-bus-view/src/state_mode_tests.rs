//! Detail/filter interaction tests for the bus log state.

use super::{BusLogState, test_support::entry};

#[test]
fn bus_detail_and_filter_modes_are_independently_cleared() {
    let mut state = BusLogState::new();
    state.push(entry("alpha event", "protocol.alpha"));

    state.enter_filter_mode();
    state.enter_detail();
    assert!(state.filter_input_mode);
    assert!(state.detail_mode);

    state.exit_filter_mode();
    assert!(!state.filter_input_mode);
    assert!(state.detail_mode);

    state.exit_detail();
    assert!(!state.detail_mode);
}

#[test]
fn bus_filter_editing_resets_selection_to_first_filtered_match() {
    let mut state = BusLogState::new();
    state.push(entry("alpha event", "protocol.alpha"));
    state.push(entry("gamma event", "protocol.gamma"));

    state.selected_index = 1;
    state.push_filter_char('m');

    assert_eq!(state.selected_index, 0);
    assert_eq!(
        state.selected_entry().map(|entry| entry.summary.as_str()),
        Some("alpha event")
    );
}
