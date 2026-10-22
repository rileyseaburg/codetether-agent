use super::*;
use codetether_provider_core::{ContentPart, Message, Role};

fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
fn node(messages: &[Message]) -> SummaryNode {
    SummaryNode {
        content: "summary".into(),
        target_tokens: 50,
        granularity: Granularity::Turn,
        generation: 0,
        provenance: Some(SummaryProvenance::new(messages, "fixture", Some("model-a")).unwrap()),
    }
}

#[test]
fn edit_in_middle_invalidates_only_affected_range() {
    let mut messages = vec![message("left"), message("middle"), message("right")];
    let mut index = SummaryIndex::new();
    for i in 0..3 {
        index.insert(
            SummaryRange::new(i, i + 1).unwrap(),
            node(&messages[i..i + 1]),
        );
    }
    messages[1] = message("change"); // Same byte length: offsets and size cannot detect this.
    assert_eq!(index.retain_valid(&messages), 1);
    assert_eq!(
        index.entries().map(|(r, _)| r.start).collect::<Vec<_>>(),
        [0, 2]
    );
}

#[test]
fn append_window_rebase_and_restart_preserve_exact_source_identity() {
    let mut messages = vec![message("prefix"), message("retained")];
    let mut index = SummaryIndex::new();
    index.insert(SummaryRange::new(1, 2).unwrap(), node(&messages[1..2]));
    messages.push(message("new turn"));
    assert_eq!(index.retain_valid(&messages), 0);
    index.evict_prefix(1);
    let mut reloaded: SummaryIndex =
        serde_json::from_str(&serde_json::to_string(&index).unwrap()).unwrap();
    assert_eq!(reloaded.retain_valid(&messages[1..]), 0);
    assert_eq!(reloaded.len(), 1);
}

#[test]
fn role_policy_and_missing_provenance_are_not_trusted() {
    let messages = vec![message("do not publish")];
    let range = SummaryRange::new(0, 1).unwrap();
    let valid = node(&messages);
    let mut changed = messages.clone();
    changed[0].role = Role::Assistant;
    assert!(!valid.valid_for(&changed, range));
    let mut legacy = valid.clone();
    legacy.provenance = None;
    assert!(!legacy.valid_for(&messages, range));
    let mut old_policy = valid;
    old_policy.provenance.as_mut().unwrap().policy_version = "obsolete".into();
    assert!(!old_policy.valid_for(&messages, range));
    assert!(!old_policy.valid_for(&messages, SummaryRange { start: 1, end: 0 }));
}

#[test]
fn legacy_json_remains_readable_and_stored_history_is_untouched() {
    let legacy = r#"{"content":"old","target_tokens":50,"granularity":"turn","generation":1}"#;
    let node: SummaryNode = serde_json::from_str(legacy).unwrap();
    let mut stored = SummaryIndex::new();
    stored.insert(SummaryRange::new(0, 1).unwrap(), node);
    let mut working = stored.clone();
    assert_eq!(working.retain_valid(&[message("source")]), 1);
    assert_eq!(stored.len(), 1);
}

#[test]
fn tool_arguments_and_results_are_part_of_identity() {
    let mut messages = vec![Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call-1".into(),
            content: "failed".into(),
        }],
    }];
    let valid = node(&messages);
    messages[0].content = vec![ContentPart::ToolResult {
        tool_call_id: "call-2".into(),
        content: "passed".into(),
    }];
    assert!(!valid.valid_for(&messages, SummaryRange::new(0, 1).unwrap()));
    let mut call = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call-1".into(),
            name: "bash".into(),
            arguments: r#"{"cmd":"cargo test"}"#.into(),
            thought_signature: None,
        }],
    }];
    let attempted = node(&call);
    if let ContentPart::ToolCall { arguments, .. } = &mut call[0].content[0] {
        *arguments = r#"{"cmd":"cargo build"}"#.into();
    }
    assert!(!attempted.valid_for(&call, SummaryRange::new(0, 1).unwrap()));
}
