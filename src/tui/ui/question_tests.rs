//! Render the production question panel using ratatui's in-memory terminal.
use super::*;
use crate::tui::app::watchdog::check_watchdog_stall;
use codetether_question::Request;
use ratatui::{Terminal, backend::TestBackend};
use serde_json::json;
use std::time::{Duration, Instant};

#[tokio::test]
async fn question_panel_renders_choices_and_suppresses_stall() {
    let mut app = App::default();
    app.state.session_id = Some("question-render".into());
    app.state.processing = true;
    app.state.main_last_event_at = Some(Instant::now() - Duration::from_secs(600));
    let request = Request::parse(json!({"questions":[{"question":"Choose a mode?",
        "header":"Mode", "options":[{"label":"Fast", "description":"Low latency"}]}]}))
    .unwrap();
    let job = tokio::spawn(runtime::scope(
        "question-render".into(),
        runtime::ask(request, None),
    ));
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while runtime::pending("question-render").is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(check_watchdog_stall(&app.state, Duration::from_secs(1)).is_none());
    let mut terminal = Terminal::new(TestBackend::new(90, 20)).unwrap();
    terminal.draw(|frame| render(frame, &app)).unwrap();
    let text: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    for expected in [
        "Choose a mode?",
        "Fast",
        "Low latency",
        "Type your own answer",
        "Esc dismiss",
    ] {
        assert!(text.contains(expected), "missing {expected}");
    }
    let mut narrow = Terminal::new(TestBackend::new(20, 8)).unwrap();
    narrow.draw(|frame| render(frame, &app)).unwrap();
    runtime::dismiss("question-render");
    assert!(job.await.unwrap().is_err());
}
