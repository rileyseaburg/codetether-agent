use super::support::draw;
use crate::tui::app::state::App;

#[test]
fn live_reasoning_visible_before_reply_and_refreshes_cached_frame() {
    let mut app = App::default();
    app.state.processing = true;
    app.state.streaming.streaming_reasoning = "Inspecting dependencies".into();
    let text = draw(&mut app, 120, 35);
    assert!(text.contains("Thinking · live"), "{text}");
    assert!(text.contains("Inspecting dependencies"), "{text}");
    assert!(app.state.streaming.streaming_text.is_empty());
    app.state.streaming.streaming_reasoning = "Found the missing import".into();
    app.state.message_cache.invalidate_streaming();
    let text = draw(&mut app, 120, 35);
    assert!(text.contains("Found the missing import"), "{text}");
    assert!(!text.contains("Inspecting dependencies"), "{text}");
}
