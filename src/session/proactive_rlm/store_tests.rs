//! Sidecar freshness tests for proactive RLM preparation.

use super::{fingerprint, store};
use crate::provider::{ContentPart, Message, Role};
use crate::session::index::SummaryIndex;

#[test]
fn schema_decoder_rejects_unknown_versions() {
    let bytes = serde_json::to_vec(&store::Prepared {
        schema_version: 99,
        message_count: 0,
        fingerprint: 0,
        generation: 0,
        index: SummaryIndex::new(),
    })
    .unwrap();
    assert!(store::decode(&bytes).is_none());
}

#[test]
fn transcript_fingerprint_changes_with_context() {
    let first = vec![message("one")];
    let second = vec![message("two")];
    assert_ne!(
        fingerprint::messages(&first),
        fingerprint::messages(&second)
    );
}

fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
