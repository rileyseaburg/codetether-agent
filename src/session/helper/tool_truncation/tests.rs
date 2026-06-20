//! Tests for truncation recovery messaging.

use super::{error_content, retry_prompt};

#[test]
fn batch_recovery_forbids_batch_retry() {
    let content = error_content("batch");
    let text = format!("{:?}", retry_prompt(&[("call-1".into(), "batch".into())]));
    assert!(content.contains("Do not call `batch`"));
    assert!(content.contains("at most three individual tool calls"));
    assert!(text.contains("Do not use `batch`"));
}

#[test]
fn write_recovery_suggests_chunked_writes() {
    let content = error_content("write");
    assert!(content.contains("output-token limit"));
    assert!(content.contains("first chunk"));
    assert!(content.contains("follow-up `edit`"));
}

#[test]
fn edit_recovery_suggests_targeted_edits() {
    let content = error_content("edit");
    assert!(content.contains("one small, targeted `edit`"));
    assert!(content.contains("function/section at a time"));
}

#[test]
fn unknown_tool_gets_generic_budget() {
    let content = error_content("bash");
    assert!(content.contains("under ~4000 characters"));
}
