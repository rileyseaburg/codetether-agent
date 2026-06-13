//! Tests for `force_keep_base` budget seeding.

use super::state_header_pins::force_keep_base;
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
fn force_keep_base_keeps_pins_outside_recent_window() {
    // 6 turns; recent window starts at index 4. Pin sits at index 0.
    let s = session(vec![
        PageKind::Constraint,
        PageKind::Conversation,
        PageKind::Conversation,
        PageKind::Conversation,
        PageKind::Conversation,
        PageKind::Conversation,
    ]);
    let mut keep = vec![false; 6];
    let per_msg = vec![10usize; 6];
    let mut budget = 1000usize;
    force_keep_base(&s, &mut keep, &per_msg, 4, &mut budget);
    // Pin at 0 kept despite being outside the recent window.
    assert!(keep[0]);
    assert!(keep[4] && keep[5]);
    assert!(!keep[1]);
    // Three kept entries × 10 tokens deducted.
    assert_eq!(budget, 970);
}
