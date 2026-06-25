//! Tests that loop-halt diagnostics are recorded into the session so an
//! autonomous worker never ends a run without a visible reason.

use super::record_loop_halt;
use crate::provider::{ContentPart, Role};
use crate::session::Session;

#[tokio::test]
async fn record_loop_halt_appends_visible_assistant_note() {
    let mut session = Session::new().await.expect("session");
    let before = session.messages.len();
    record_loop_halt(&mut session, "step budget (50) exhausted before completion").await;
    assert_eq!(session.messages.len(), before + 1);
    let last = session.messages.last().expect("message");
    assert!(matches!(last.role, Role::Assistant));
    let text = last.content.iter().find_map(|p| match p {
        ContentPart::Text { text } => Some(text.as_str()),
        _ => None,
    });
    let text = text.expect("text part");
    assert!(text.contains("Agent loop halted"));
    assert!(text.contains("step budget"));
}
