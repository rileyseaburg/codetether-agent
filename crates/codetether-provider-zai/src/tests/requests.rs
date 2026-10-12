//! Thinking and generation-parameter regression coverage.
use serde_json::json;

#[test]
fn exact_requests_disable_thinking() {
    let request = super::request("Reply with exactly: api-model-call-ok", 20);
    assert_eq!(
        crate::thinking::thinking_config_for_request(&request),
        json!({"type":"disabled"})
    );
}

#[test]
fn long_low_budget_requests_keep_thinking() {
    let text = format!(
        "Summarize the following material. {}",
        "context paragraph. ".repeat(40)
    );
    let request = super::request(&text, 128);
    assert_eq!(
        crate::thinking::thinking_config_for_request(&request)["type"],
        "enabled"
    );
}

#[test]
fn normal_requests_keep_clear_thinking_and_generation_limits() {
    let mut request = super::request("Explain the tradeoffs.", 1024);
    request.top_p = Some(0.9);
    request.stop = vec!["STOP".into()];
    let wire = crate::request::body(&request);
    assert_eq!(
        wire["thinking"],
        json!({"type":"enabled", "clear_thinking":true})
    );
    assert_eq!(wire["max_tokens"], 1024);
    assert_eq!(wire["top_p"], json!(request.top_p.unwrap()));
    assert_eq!(wire["stop"], json!(["STOP"]));
    assert_eq!(wire["stream"], false);
}
