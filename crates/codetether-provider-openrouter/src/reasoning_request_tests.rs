use super::{apply, reasoning_object};
use serde_json::json;

#[test]
fn builds_effort_object_for_a_valid_level() {
    let object = reasoning_object("x-ai/grok-4.6", Some("high")).expect("reasoning object");
    assert_eq!(object, json!({"effort": "high"}));
}

#[test]
fn omits_reasoning_when_no_override_is_configured() {
    assert!(reasoning_object("x-ai/grok-4.6", None).is_none());
}

#[test]
fn omits_reasoning_for_a_level_openrouter_would_reject() {
    assert!(reasoning_object("x-ai/grok-4.6", Some("ultra")).is_none());
}

#[test]
fn omits_none_for_models_that_mandate_reasoning() {
    // Grok 4.6 answers HTTP 400 "Reasoning is mandatory for this endpoint".
    assert!(reasoning_object("x-ai/grok-4.6", Some("none")).is_none());
}

#[test]
fn still_sends_none_where_the_model_permits_it() {
    let object = reasoning_object("x-ai/grok-4.3", Some("none")).expect("reasoning object");
    assert_eq!(object, json!({"effort": "none"}));
}

#[test]
fn apply_sets_effort_and_requests_reasoning_passthrough() {
    let mut body = json!({"model": "x-ai/grok-4.6"});
    apply(&mut body, "x-ai/grok-4.6", Some("xhigh"));
    assert_eq!(body["reasoning"], json!({"effort": "xhigh"}));
    assert_eq!(body["include_reasoning"], json!(true));
}

#[test]
fn apply_leaves_the_body_untouched_without_a_usable_level() {
    let mut body = json!({"model": "x-ai/grok-4.6"});
    apply(&mut body, "x-ai/grok-4.6", None);
    assert!(body.get("reasoning").is_none());
    assert!(body.get("include_reasoning").is_none());
}
