use std::time::{Duration, Instant};

use super::{finish, ready};
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
    let prior = Instant::now() - Duration::from_millis(67);
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

#[test]
fn skipped_frame_remains_dirty_for_retry() {
    let mut app = App::default();
    app.state.needs_redraw = true;
    let prior = Instant::now();
    let mut last_draw = Some(prior);

    finish(&mut app, &mut last_draw, false);

    assert!(app.state.needs_redraw);
    assert_eq!(last_draw, Some(prior));
}
