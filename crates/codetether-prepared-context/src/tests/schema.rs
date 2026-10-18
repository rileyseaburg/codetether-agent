//! Sidecar freshness tests for proactive RLM preparation.

use crate::{Prepared, decode, fingerprint};
use codetether_context_policy::summary_index::SummaryIndex;
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn schema_decoder_rejects_unknown_versions() {
    let bytes = serde_json::to_vec(&Prepared {
        schema_version: 99,
        message_count: 0,
        fingerprint: 0,
        generation: 0,
        index: SummaryIndex::new(),
    })
    .unwrap();
    assert!(decode(&bytes).is_none());
}

#[test]
fn transcript_fingerprint_changes_with_context() {
    let first = vec![message("one")];
    let second = vec![message("two")];
    assert_ne!(fingerprint(&first), fingerprint(&second));
}

fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
