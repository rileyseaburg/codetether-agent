use super::{build_body, error_chunks};
use crate::provider::{CompletionRequest, StreamChunk};
use serde_json::json;

fn request(model: &str) -> CompletionRequest {
    CompletionRequest {
        messages: vec![],
        tools: vec![],
        model: model.into(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: vec![],
    }
}

#[test]
fn omits_tools_when_none_supplied() {
    let body = build_body(&request("glm-5.2"), vec![], vec![], json!({}), true);

    assert!(body.get("tools").is_none());
    assert!(body.get("tool_stream").is_none());
}

#[test]
fn tool_stream_flag_respects_model_support() {
    let tool = json!({"type": "function"});

    let on = build_body(
        &request("glm-5.2"),
        vec![],
        vec![tool.clone()],
        json!({}),
        true,
    );
    assert_eq!(on["tool_stream"], true);

    let off = build_body(&request("glm-4"), vec![], vec![tool], json!({}), false);
    assert!(off.get("tool_stream").is_none());
}

#[test]
fn defaults_temperature_to_one_and_marks_stream() {
    let body = build_body(&request("glm-5.2"), vec![], vec![], json!({}), false);

    assert_eq!(body["temperature"], 1.0);
    assert_eq!(body["stream"], true);
}

#[test]
fn error_response_yields_error_then_done() {
    let chunks = error_chunks("boom".into());

    assert_eq!(chunks.len(), 2);
    assert!(matches!(&chunks[0], StreamChunk::Error(m) if m == "boom"));
    assert!(matches!(chunks[1], StreamChunk::Done { usage: None }));
}
