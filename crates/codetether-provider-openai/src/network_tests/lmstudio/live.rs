//! Explicit opt-in probe of the new provider against an operator-supplied local endpoint.
use super::*;
#[tokio::test]
#[ignore = "requires CODETETHER_LMSTUDIO_LIVE_BASE and a loaded Gemma model"]
async fn lmstudio_live_output_above_old_ceiling() {
    let base = std::env::var("CODETETHER_LMSTUDIO_LIVE_BASE").expect("explicit endpoint required");
    let provider = provider(base);
    let models = provider.list_models().await.unwrap();
    let model = models
        .iter()
        .find(|model| model.id == MODEL)
        .expect("Gemma must be loaded");
    assert!(model.context_window >= 8192);
    assert!(model.max_output_tokens.unwrap() > 1024);
    let mut input = request(MODEL);
    input.max_tokens = Some(1200);
    input.temperature = Some(0.0);
    input.messages[0].content = vec![ContentPart::Text {
        text: "Count from 1 to 1000, one number per line. Output numbers only. Keep going until you reach 1000; do not abbreviate or summarize.".into(),
    }];
    let response = provider.complete(input).await.unwrap();
    assert!(
        response.usage.completion_tokens > 1024,
        "probe must exercise the raised ceiling"
    );
    assert!(response.usage.completion_tokens <= 1200);
    assert!(response.usage.total_tokens <= model.context_window);
    let text: String = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert!(!text.is_empty());
    eprintln!(
        "LIVE_LMSTUDIO model={MODEL} context={} advertised_output={} prompt_tokens={} completion_tokens={} total_tokens={} finish={:?} response_chars={}",
        model.context_window,
        model.max_output_tokens.unwrap(),
        response.usage.prompt_tokens,
        response.usage.completion_tokens,
        response.usage.total_tokens,
        response.finish_reason,
        text.len()
    );
}
