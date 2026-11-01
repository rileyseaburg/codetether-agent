//! Pure policy tests do not mutate process-global environment variables.
use super::super::budget;
use super::*;
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};

#[test]
fn lmstudio_configuration_rejects_invalid_values() {
    assert_eq!(parse("cap", "4096", 1, 32768).unwrap(), 4096);
    for value in ["0", "-1", "garbage", "32769", ""] {
        assert!(parse("cap", value, 1, 32768).is_err());
    }
    assert_eq!(parse("queue", "0", 0, 128).unwrap(), 0);
}
#[test]
fn lmstudio_output_ceiling_respects_loaded_capacity_and_configuration() {
    assert_eq!(budget::output_ceiling(32768, 4096), 4096);
    assert_eq!(budget::output_ceiling(32768, 8192), 8192);
    assert_eq!(budget::output_ceiling(4096, 8192), 1024);
    assert_eq!(budget::output_ceiling(32768, 512), 512);
}
#[test]
fn lmstudio_explicit_smaller_output_is_preserved() {
    let mut request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "hello".into(),
            }],
        }],
        tools: vec![],
        model: "gemma".into(),
        temperature: None,
        top_p: None,
        max_tokens: Some(64),
        stop: vec![],
    };
    budget::cap_output(&mut request, 32768, 4096).unwrap();
    assert_eq!(request.max_tokens, Some(64));
}
