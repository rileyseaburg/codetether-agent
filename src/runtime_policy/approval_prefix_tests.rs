use super::from_args;
use serde_json::json;

#[test]
fn amendment_must_prefix_the_reviewed_command() {
    let args = json!({
        "command": "cargo test --lib approval",
        "prefix_rule": ["cargo", "test"]
    });
    let amendment = from_args("bash", &args).expect("matching prefix");
    assert_eq!(amendment.command(), &["cargo", "test"]);
}

#[test]
fn unrelated_or_partial_word_prefix_is_rejected() {
    for args in [
        json!({"command": "echo reviewed", "prefix_rule": ["rm"]}),
        json!({"command": "cargo-test", "prefix_rule": ["cargo"]}),
        json!({"command": "echo ok && rm file", "prefix_rule": ["echo", "ok", "&&"]}),
    ] {
        assert!(from_args("bash", &args).is_none());
    }
}
