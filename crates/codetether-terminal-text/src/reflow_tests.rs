use super::strip_tui_artifacts;

#[test]
fn reflows_wrapped_tui_rows() {
    let raw = "│ clipboard text wrapped across │\n│ two rendered terminal rows   │";

    assert_eq!(
        strip_tui_artifacts(raw),
        "clipboard text wrapped across two rendered terminal rows"
    );
}

#[test]
fn preserves_blank_row_as_paragraph_break() {
    let raw = "│ first paragraph  │\n│                  │\n│ second paragraph │";

    assert_eq!(
        strip_tui_artifacts(raw),
        "first paragraph\n\nsecond paragraph"
    );
}
