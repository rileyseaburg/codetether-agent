use std::time::{Duration, Instant};

use super::ready;
use crate::tui::app::state::App;

#[test]
fn first_and_idle_frames_are_immediate() {
    let mut app = App::default();
    assert!(ready(&app, None));
    assert!(ready(&app, Some(Instant::now())));
    app.state.processing = true;
    assert!(!ready(&app, Some(Instant::now())));
}

#[test]
fn streaming_frame_is_ready_after_interval() {
    let mut app = App::default();
    app.state.processing = true;
    let prior = Instant::now() - Duration::from_millis(34);
    assert!(ready(&app, Some(prior)));
}

#[test]
fn keyboard_input_bypasses_streaming_limit() {
    let mut app = App::default();
    app.state.processing = true;
    let prior = Instant::now();
    app.state.last_key_at = Some(prior + Duration::from_millis(1));
    assert!(ready(&app, Some(prior)));
}
