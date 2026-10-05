//! Render satisfaction UI in a mocked local ratatui terminal.

use ratatui::{Terminal, backend::TestBackend, style::Color};
#[path = "../app/event_handlers/answer_review_test_support.rs"]
mod support;

#[tokio::test]
async fn answer_review_popup_has_explicit_yes_no_and_defaults_no() {
    let (app, _slot) = support::ready().await;
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    terminal.draw(|frame| super::render(frame, &app)).unwrap();
    let text: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect();
    assert!(text.contains("Are you satisfied with the answer?"));
    assert!(text.contains("[ Yes ]"));
    assert!(text.contains("[ No ]"));
    let lines = super::content::lines(false);
    assert_eq!(lines[2].spans[2].style.fg, Some(Color::Cyan));
    assert_ne!(lines[2].spans[0].style.fg, Some(Color::Cyan));
}
