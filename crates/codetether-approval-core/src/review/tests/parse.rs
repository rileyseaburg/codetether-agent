//! Verdict extraction from free-form model output.
use crate::review::{ReviewOutcome, parse};

#[test]
fn parse_takes_the_last_json_object_and_tolerates_prose() {
    let text = "I opened the file. {\"note\":\"x\"} Conclusion:\n\
        ```json\n{\"outcome\":\"request_changes\",\"reason\":\"Duplicates for_workspace.\",\"findings\":[\"see registry/key.rs\"]}\n```";
    let verdict = parse(text);
    assert_eq!(verdict.outcome, ReviewOutcome::RequestChanges);
    assert_eq!(verdict.findings, vec!["see registry/key.rs"]);
}

#[test]
fn parse_ignores_braces_inside_strings() {
    let text = r#"{"outcome":"approve","reason":"handles `{` in match arms","findings":[]}"#;
    assert_eq!(parse(text).outcome, ReviewOutcome::Approve);
}

#[test]
fn parse_without_a_verdict_escalates_with_the_tail() {
    let verdict = parse("I ran out of ideas.");
    assert_eq!(verdict.outcome, ReviewOutcome::Escalate);
    assert!(verdict.reason.contains("ran out of ideas"));
}
