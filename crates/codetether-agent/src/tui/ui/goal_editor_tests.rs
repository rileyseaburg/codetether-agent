//! Render the prefilled goal editor and its actions in a mocked terminal.
use crate::session::tasks::answer_review_test_support;
use crate::tui::app::{input::goal_answer::editor, session_runtime::SessionView, state::App};
use ratatui::{Terminal, backend::TestBackend};

#[tokio::test]
async fn goal_editor_renders_prefilled_text_and_save_cancel_controls() {
    let session = answer_review_test_support::session().await;
    let mut app = App::default();
    app.state.session_id = Some(session.id.clone());
    editor::open(&mut app, &session.id);
    let mut terminal = Terminal::new(TestBackend::new(100, 20)).unwrap();
    terminal
        .draw(|frame| super::super::main::ui(frame, &mut app, &SessionView::default()))
        .unwrap();
    let text: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect();
    assert!(text.contains("Session goal (draft)"));
    assert!(text.contains("Finish the goal"));
    assert!(text.contains("Ctrl+S save goal"));
    assert!(text.contains("Esc discard"));
    app.state.status = "Goal changed while editing; draft retained".into();
    terminal
        .draw(|frame| super::render(frame, &mut app))
        .unwrap();
    let text: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect();
    assert!(text.contains("Goal changed while editing; draft retained"));
    editor::close(&mut app);
}
