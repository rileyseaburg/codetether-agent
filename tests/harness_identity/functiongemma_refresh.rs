//! Refresh stale dedicated metadata without changing embedded caller examples.

use super::identity_prompt::with_identity;

#[test]
fn functiongemma_identity_switch_is_fresh_and_idempotent() {
    let old = crate::identity::system_prompt("", "old-provider", "old-model");
    let caller = "Extract tool calls only.\nExample follows:\n";
    let rest = "\n<start_of_turn>user\nDo not change\n<end_of_turn>\n<start_of_turn>model\n";
    let prompt =
        format!("<start_of_turn>system\n{old}\n\n{old}\n\n{caller}{old}<end_of_turn>{rest}");
    let first = with_identity(&prompt, "first.gguf");
    let second = with_identity(&first, "second.gguf");
    let preserved = format!("{caller}{old}");
    let expected = crate::identity::system_prompt(&preserved, "candle", "second.gguf");
    assert_eq!(
        second,
        format!("<start_of_turn>system\n{expected}<end_of_turn>{rest}")
    );
    assert!(!second.contains("first.gguf"));
    assert_eq!(second.matches("<codetether-harness-identity>").count(), 2);
    assert_eq!(with_identity(&second, "second.gguf"), second);
}

#[test]
fn functiongemma_identity_preserves_caller_owned_lookalike() {
    let lookalike = "<codetether-harness-identity>caller example</codetether-harness-identity>";
    let prompt = format!("<start_of_turn>system\n{lookalike}<end_of_turn>\nuser content");
    let actual = with_identity(&prompt, "checkpoint.gguf");
    let expected = crate::identity::system_prompt(lookalike, "candle", "checkpoint.gguf");
    assert_eq!(
        actual,
        format!("<start_of_turn>system\n{expected}<end_of_turn>\nuser content")
    );
}
