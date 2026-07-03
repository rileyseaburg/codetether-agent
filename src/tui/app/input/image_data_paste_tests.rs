//! Tests for embedded image data URL extraction and submit-time draining.

use base64::Engine;

use super::drain_embedded_images;
use super::extract::extract_image_data_url;
use crate::tui::app::state::App;

fn png_data_url() -> String {
    let payload = base64::engine::general_purpose::STANDARD.encode("png bytes");
    format!("data:image/png;base64,{payload}")
}

#[test]
fn extracts_data_url_surrounded_by_text() {
    let url = png_data_url();
    let text = format!("look at this {url} please");
    let found = extract_image_data_url(&text).expect("should find url");
    assert_eq!(found.data_url, url);
    assert_eq!(found.remainder, "look at this  please".trim());
}

#[test]
fn returns_none_without_data_url() {
    assert!(extract_image_data_url("just a normal prompt").is_none());
}

#[test]
fn drain_attaches_image_and_keeps_caption() {
    let mut app = App::default();
    let url = png_data_url();
    app.state.input = format!("describe {url}");
    app.state.input_cursor = app.state.input.chars().count();
    let n = drain_embedded_images(&mut app);
    assert_eq!(n, 1);
    assert_eq!(app.state.pending_images.len(), 1);
    assert_eq!(app.state.input, "describe");
}

#[test]
fn drain_handles_pure_image_paste() {
    let mut app = App::default();
    app.state.input = png_data_url();
    let n = drain_embedded_images(&mut app);
    assert_eq!(n, 1);
    assert_eq!(app.state.pending_images.len(), 1);
    assert!(app.state.input.is_empty());
}
