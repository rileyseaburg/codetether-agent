//! Prepared-summary fixtures with known coverage and rendered token cost.
use super::super::{SummaryGap, summary_message};
use codetether_context_policy::summary_index::{
    Granularity, SummaryIndex, SummaryNode, SummaryRange,
};
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_request_tokens::estimate_tokens_for_messages;

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
            provenance: Some(
                codetether_context_policy::summary_index::SummaryProvenance::new(
                    &transcript()[..2],
                    "fixture",
                    None,
                )
                .unwrap(),
            ),
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
