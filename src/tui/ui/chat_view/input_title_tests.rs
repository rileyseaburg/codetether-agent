//! Phone-width input title tests.

use super::input_title::build;
use crate::tui::app::state::App;

#[test]
fn phone_width_uses_compact_title() {
    let app = App::default();
    let title = build(&app, "", 44);
    assert!(title.chars().count() <= 44, "title overflows phone width: {title:?}");
    assert!(!title.contains("Ctrl+O"), "{title:?}");
}

#[test]
fn desktop_width_keeps_full_shortcuts() {
    let app = App::default();
    assert!(build(&app, "", 160).contains("Ctrl+O=copy reply"));
}
