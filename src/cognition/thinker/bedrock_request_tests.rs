//! Request-level regressions for direct Bedrock thinking, without credentials.
use super::{ThinkerConfig, body, url};
use crate::provider::metrics::identity::prompt;

#[test]
fn thinker_identity_bedrock_refreshes_and_preserves_converse_fields() {
    let mut config = ThinkerConfig {
        model: "us.anthropic.claude-sonnet-4-20250514-v1:0".into(),
        bedrock_service_tier: Some("priority".into()),
        temperature: 0.4,
        max_tokens: 91,
        ..ThinkerConfig::default()
    };
    let caller = "  Caller instructions.\nKeep spacing.  ";
    let user = "Who are you?";
    let first = body(&config, caller, user);
    assert_eq!(first["system"][0]["text"], prompt("bedrock", &config.model));
    config.model = "us.anthropic.claude-opus-4-20250514-v1:0".into();
    let stale = first["system"][0]["text"].as_str().unwrap();
    let next = body(&config, &format!("{stale}\n\n{stale}\n\n{caller}"), user);
    assert_eq!(next["system"][0]["text"], prompt("bedrock", &config.model));
    assert_eq!(next["system"][1]["text"], caller);
    assert_eq!(next["system"].as_array().unwrap().len(), 2);
    assert_eq!(next["messages"][0]["role"], "user");
    assert_eq!(next["messages"][0]["content"][0]["text"], user);
    assert_eq!(next["inferenceConfig"]["maxTokens"], 91);
    assert_eq!(next["inferenceConfig"]["temperature"], 0.4_f32);
    assert_eq!(
        next["additionalModelRequestFields"]["service_tier"],
        "priority"
    );
    assert!(url(&config).ends_with(&format!("/model/{}/converse", config.model)));
    assert!(!url(&config).contains("%3A"));
}

#[test]
fn thinker_identity_bedrock_preserves_embedded_metadata() {
    let caller = format!("Caller example:\n{}", prompt("quoted", "example"));
    let request = body(&ThinkerConfig::default(), &caller, "");
    assert_eq!(request["system"][1]["text"], caller);
    assert!(request.get("additionalModelRequestFields").is_none());
}
