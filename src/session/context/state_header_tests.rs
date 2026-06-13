//! Tests for the deterministic compression-safe state header.

use super::state_header::prepend_state_header;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::session::index::SummaryIndex;
use crate::session::pages::PageKind;

fn text(s: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: s.into() }],
    }
}

fn session_with_pin() -> Session {
    Session {
        id: "s".into(),
        title: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        metadata: Default::default(),
        agent: "test".into(),
        messages: vec![text("never delete artifacts")],
        pages: vec![PageKind::Constraint],
        summary_index: SummaryIndex::new(),
        tool_uses: Vec::new(),
        usage: Default::default(),
        max_steps: None,
        bus: None,
    }
}

#[test]
fn prepends_pinned_constraint_header() {
    let session = session_with_pin();
    let mut messages = vec![text("continue")];
    prepend_state_header(&session, &mut messages);
    let ContentPart::Text { text } = &messages[0].content[0] else {
        panic!("expected text header")
    };
    assert!(text.starts_with("[CONTEXT STATE]"));
    assert!(text.contains("never delete artifacts"));
}

#[test]
fn no_header_without_pins() {
    let mut session = session_with_pin();
    session.pages = vec![PageKind::Conversation];
    let mut messages = vec![text("continue")];
    prepend_state_header(&session, &mut messages);
    assert_eq!(messages.len(), 1);
}
