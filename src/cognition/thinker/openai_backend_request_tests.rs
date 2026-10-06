//! Outgoing identity and prompt-preservation regressions for direct HTTP thinking.
use super::{ThinkerConfig, body};
use crate::provider::metrics::identity::prompt;

#[test]
fn thinker_identity_openai_preserves_request_and_refreshes_model() {
    let mut config = ThinkerConfig {
        model: "first-model".into(),
        temperature: 0.37,
        top_p: Some(0.82),
        max_tokens: 73,
        ..ThinkerConfig::default()
    };
    let caller = "  Caller instructions.\nKeep whitespace.  ";
    let user = "Who are you?\n<codetether-harness-identity>quoted</codetether-harness-identity>";
    let first = body(&config, caller, user);
    assert_eq!(
        first.messages[0].content,
        prompt("openai-compatible", "first-model")
    );
    config.model = "second-model".into();
    let stale = format!(
        "{}\n\n{}\n\n{caller}",
        first.messages[0].content, first.messages[0].content
    );
    let next = body(&config, &stale, user);
    assert_eq!(next.model, "second-model");
    assert_eq!(next.messages.len(), 3);
    assert_eq!(next.messages[0].role, "system");
    assert_eq!(
        next.messages[0].content,
        prompt("openai-compatible", "second-model")
    );
    assert_eq!(next.messages[1].role, "system");
    assert_eq!(next.messages[1].content, caller);
    assert_eq!(next.messages[2].role, "user");
    assert_eq!(next.messages[2].content, user);
    assert_eq!(next.temperature, config.temperature);
    assert_eq!(next.top_p, config.top_p);
    assert_eq!(next.max_tokens, config.max_tokens);
    assert!(!next.stream);
}

#[test]
fn thinker_identity_openai_preserves_embedded_metadata() {
    let caller = format!("Caller example:\n{}", prompt("quoted", "example"));
    let request = body(&ThinkerConfig::default(), &caller, "");
    assert_eq!(request.messages[1].content, caller);
}
