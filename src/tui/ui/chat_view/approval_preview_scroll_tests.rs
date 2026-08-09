//! Regression coverage for wrapped-row approval preview scrolling.

/// The exact failure from the screenshot: one long `printf | curl` command
/// pinned at "line 1" because logical-line counting reported no scrollback.
const LONG_COMMAND: &str = "printf '%s' '{\"component_type\":\"playwright_no_code_hero_1783389134535\",\"config\":{\"headline\":\"Runtime authored, no rebuild\",\"cta\":\"Book now\"}}' | curl -sS -w '\\nHTTP:%{http_code}\\n' --data-binary @- http://192.168.50.101:8081/api/v1/components";

#[test]
fn single_long_line_wraps_to_many_rows() {
    assert!(LONG_COMMAND.lines().count() == 1);

    assert!(super::wrapped_rows(LONG_COMMAND, 80) > 1);
}

#[test]
fn long_single_line_command_is_scrollable() {
    // A short pane, as in the reported overlay: the wrapped command
    // overflows it even though `lines().count()` is 1, which is exactly
    // what previously pinned the preview to row 1.
    let rows = super::wrapped_rows(LONG_COMMAND, 80);
    let height = u16::try_from(rows - 1).unwrap();

    assert!(
        super::max_offset(LONG_COMMAND, 80, height) > 0,
        "one-line command must scroll when it overflows the pane"
    );
}

#[test]
fn offset_stops_with_content_still_visible() {
    let content = "a\n".repeat(40);

    assert_eq!(super::max_offset(&content, 80, 10), 30);
}

#[test]
fn content_shorter_than_viewport_does_not_scroll() {
    assert_eq!(super::max_offset("one\ntwo", 80, 10), 0);
}

#[test]
fn narrow_pane_wraps_more_aggressively() {
    let narrow = super::wrapped_rows(LONG_COMMAND, 40);
    let wide = super::wrapped_rows(LONG_COMMAND, 120);

    assert!(narrow > wide);
}
