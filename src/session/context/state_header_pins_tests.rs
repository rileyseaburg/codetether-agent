//! Tests for pinned-constraint force-keep helpers.

use super::state_header_pins::pinned_indices;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::session::index::SummaryIndex;
use crate::session::pages::PageKind;

fn msg(s: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: s.into() }],
    }
}

fn session(pages: Vec<PageKind>) -> Session {
    let messages = (0..pages.len()).map(|i| msg(&format!("m{i}"))).collect();
    Session {
        id: "s".into(),
        title: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        metadata: Default::default(),
        agent: "t".into(),
        messages,
        pages,
        summary_index: SummaryIndex::new(),
        tool_uses: Vec::new(),
        usage: Default::default(),
        max_steps: None,
        bus: None,
    }
}

#[test]
fn pinned_indices_finds_constraints() {
    let s = session(vec![
        PageKind::Conversation,
        PageKind::Constraint,
        PageKind::Conversation,
        PageKind::Constraint,
    ]);
    assert_eq!(pinned_indices(&s), vec![1, 3]);
}
