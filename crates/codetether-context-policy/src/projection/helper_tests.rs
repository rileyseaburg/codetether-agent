//! Existing derived-context record regressions.

use super::*;

use codetether_provider_core::{ContentPart, Role};

#[test]
fn derived_context_record_round_trips() {
    let ctx = DerivedContext {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hi".to_string(),
            }],
        }],
        origin_len: 1,
        compressed: false,
        dropped_ranges: Vec::new(),
        provenance: Vec::new(),
        resolutions: vec![ResidencyLevel::Full],
    };
    let cloned = ctx.clone();
    assert_eq!(ctx.origin_len, cloned.origin_len);
    assert_eq!(ctx.compressed, cloned.compressed);
    assert_eq!(ctx.messages.len(), cloned.messages.len());
    assert_eq!(ctx.resolutions, cloned.resolutions);
}

#[test]
fn messages_len_changed_detects_shrink_and_noop() {
    let empty: Vec<Message> = Vec::new();
    assert!(!messages_len_changed(0, &empty));

    let one = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "x".to_string(),
        }],
    }];
    assert!(messages_len_changed(5, &one));
    assert!(!messages_len_changed(1, &one));
}
