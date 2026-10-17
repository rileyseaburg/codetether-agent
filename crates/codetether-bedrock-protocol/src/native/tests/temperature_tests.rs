//! Native InvokeModel temperature capability regressions.

use super::body_tests::req;
use crate::native::body::build_anthropic_messages_body;

#[test]
fn haiku_55_native_body_omits_even_explicit_temperature() {
    let mut request = req(vec![], vec![]);
    request.temperature = Some(0.7);

    let body = build_anthropic_messages_body(&request, "us.anthropic.claude-haiku-5-5");

    assert!(body.get("temperature").is_none(), "{body}");
}

#[test]
fn supported_native_model_keeps_explicit_temperature() {
    let mut request = req(vec![], vec![]);
    request.temperature = Some(0.7);

    let body =
        build_anthropic_messages_body(&request, "us.anthropic.claude-haiku-4-5-20251001-v1:0");

    let temperature = body["temperature"].as_f64().expect("temperature number");
    assert_eq!(temperature, f64::from(0.7_f32));
}
