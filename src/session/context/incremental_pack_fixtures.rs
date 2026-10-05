//! Prepared-summary fixtures with known coverage and rendered token cost.
use super::super::{SummaryGap, summary_message};
use crate::provider::{ContentPart, Message, Role};
use crate::session::helper::token::estimate_tokens_for_messages;
use crate::session::index::{Granularity, SummaryIndex, SummaryNode, SummaryRange};

pub(super) fn prepared() -> (SummaryIndex, usize) {
    let range = SummaryRange::new(0, 2).unwrap();
    let content = "decision: preserve the existing API".to_string();
    let cost = estimate_tokens_for_messages(&[summary_message(&SummaryGap {
        range,
        content: content.clone(),
    })]);
    let mut index = SummaryIndex::new();
    index.insert(
        range,
        SummaryNode {
            content,
            target_tokens: 512,
            granularity: Granularity::Phase,
            generation: 1,
        },
    );
    (index, cost)
}

pub(super) fn transcript() -> Vec<Message> {
    let raw = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: "raw work ".repeat(1000),
        }],
    };
    let active = Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Implement the next feature without changing its API".into(),
        }],
    };
    vec![raw.clone(), raw, active]
}
