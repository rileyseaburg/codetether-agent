//! Tests for image data-URL paste rejection handling.

use super::try_attach_data_url;
use crate::tui::app::state::App;

#[test]
fn oversized_image_paste_reports_error_and_is_consumed() {
    let mut app = App::default();
    // Over the 10 MB decoded cap: must not land in the text sidecar.
    let huge = format!("data:image/png;base64,{}", "A".repeat(15_000_000));

    assert!(
        try_attach_data_url(&mut app, &huge),
        "rejected image must be consumed, not passed to the paste sidecar"
    );
    assert!(app.state.pending_images.is_empty());
    assert!(
        app.state.status.contains("too large"),
        "status should explain the failure, got {:?}",
        app.state.status
    );
}

#[test]
fn unsupported_image_type_is_reported() {
    let mut app = App::default();
    let text = "data:image/tiff;base64,AAAA";
    assert!(try_attach_data_url(&mut app, text));
    assert!(app.state.status.contains("Unsupported image type"));
}

#[test]
fn plain_text_paste_is_not_consumed_as_image() {
    let mut app = App::default();
    let before = app.state.status.clone();
    assert!(!try_attach_data_url(&mut app, "just a normal prompt"));
    assert_eq!(
        app.state.status, before,
        "non-image paste must not overwrite the status line"
    );
}
