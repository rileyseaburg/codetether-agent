use super::super::*;
use crate::session::SessionEvent;
use crate::tui::chat::message::MessageType;

#[tokio::test]
async fn thinking_complete_keeps_full_scrollable_text() {
    let mut app = App::default();
    let mut slot = super::test_slot().await;
    let full = format!("{}tail", "reasoning line\n".repeat(80));
    handle_session_event(
        &mut app,
        &mut slot,
        &None,
        SessionEvent::ThinkingComplete(full.clone()),
    )
    .await;

    let message = app.state.messages.last().expect("thinking message");
    assert!(matches!(message.message_type, MessageType::Thinking(_)));
    assert_eq!(message.content, full);
    assert!(message.content.len() > 600);
}
