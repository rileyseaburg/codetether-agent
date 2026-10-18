use super::visible_range;

#[test]
fn window_tracks_selection_without_formatting_all_entries() {
    assert_eq!(visible_range(500, 0, 20), (0, 20));
    assert_eq!(visible_range(500, 250, 20), (231, 251));
    assert_eq!(visible_range(500, 499, 20), (480, 500));
}

#[test]
fn window_handles_empty_and_zero_height() {
    assert_eq!(visible_range(0, 0, 20), (0, 0));
    assert_eq!(visible_range(5, 2, 0), (2, 2));
}
