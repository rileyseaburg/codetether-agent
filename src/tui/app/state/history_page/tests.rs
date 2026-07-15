//! Paging order, boundary, and rendered-anchor tests.

use crate::provider::{ContentPart, Message, Role};
use crate::tui::app::state::AppState;

use super::select::Decision;
use super::types::Request;
use super::{anchor, select};

#[test]
fn selects_preceding_messages_in_chronological_order() {
    let messages: Vec<_> = (0..300).map(message).collect();
    let request = Request {
        generation: 1,
        source_id: "unused".into(),
        boundary: anchor::fingerprints(&messages[275..]),
        depth: 25,
    };
    let Decision::Ready(page) = select::page(&messages, &request, 400) else {
        panic!()
    };
    assert_eq!(text(&page.messages[0]), "25");
    assert_eq!(text(&page.messages[249]), "274");
}

#[test]
fn prepend_anchor_moves_back_by_requested_rows() {
    let mut state = AppState::default();
    state.history_page.pending_old_lines = Some(100);
    state.history_page.pending_old_scroll = 0;
    state.history_page.pending_render_rewind = 19;
    state.apply_pending_history_anchor(160);
    assert_eq!(state.chat_scroll, 41);
    assert!(!state.chat_auto_follow);
}

fn message(value: usize) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: value.to_string(),
        }],
    }
}

fn text(message: &Message) -> &str {
    match &message.content[0] {
        ContentPart::Text { text } => text,
        _ => unreachable!(),
    }
}
