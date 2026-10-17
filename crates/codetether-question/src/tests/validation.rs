//! validation contract tests.
use crate::Request;
use serde_json::json;
#[test]
fn validates_batches_and_legacy_defaults() {
    for value in [
        json!({"questions":[]}),
        json!({"question":" "}),
        json!({"questions":[{"question":"Which?", "options":[{"label":"A"},{"label":"A"}]}]}),
        json!({"questions":[{"question":"Which?", "options":[{"label":""}]}]}),
        json!({"questions":[{"question":"Which?", "header":"x".repeat(31)}]}),
        json!({"questions":null}),
    ] {
        assert!(Request::parse(value).is_err());
    }
    let old =
        Request::parse(json!({"question":"Which?", "options":["A","B"], "default":"B"})).unwrap();
    assert_eq!(old.questions()[0].options[0].label, "B");
    assert_eq!(old.questions()[0].options.len(), 2);
}
