//! Live OpenRouter checks for Grok 4.6 reasoning levels.
//!
//! These tests call the real router through `OpenRouterProvider`, so they are
//! skipped unless `OPENROUTER_API_KEY` is set. Run with:
//!
//! ```text
//! OPENROUTER_API_KEY=... cargo test --test openrouter_grok_thinking_live -- --nocapture
//! ```

use codetether_agent::provider::openrouter::{OpenRouterProvider, runtime_config};
use codetether_agent::provider::{
    CompletionRequest, ContentPart, Message, Provider, Role, StreamChunk,
};
use futures::StreamExt;

const MODEL: &str = "x-ai/grok-4.6";

fn api_key() -> Option<String> {
    std::env::var("OPENROUTER_API_KEY")
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
}

fn request(prompt: &str) -> CompletionRequest {
    CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_string(),
            }],
        }],
        tools: Vec::new(),
        model: MODEL.to_string(),
        temperature: None,
        top_p: None,
        max_tokens: Some(2048),
        stop: Vec::new(),
    }
}

/// Every level the router advertises must produce a usable answer.
#[tokio::test]
async fn grok_4_6_answers_at_each_thinking_level() {
    let Some(key) = api_key() else {
        eprintln!("skipping: OPENROUTER_API_KEY not set");
        return;
    };
    let provider = OpenRouterProvider::new(key).expect("provider builds");
    let previous = runtime_config::thinking_level();

    for level in ["minimal", "low", "medium", "high", "xhigh", "max"] {
        runtime_config::set_thinking_level(Some(level.to_string()));
        let response = provider
            .complete(request("What is 17*23? Reply with only the number."))
            .await
            .unwrap_or_else(|e| panic!("level {level} failed: {e}"));

        let text: String = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert!(
            text.contains("391"),
            "level {level} gave wrong answer: {text:?}"
        );
        eprintln!("level {level}: ok ({} chars)", text.len());
    }

    runtime_config::set_thinking_level(previous);
}

/// `none` must not fail the turn even though Grok 4.6 rejects it upstream.
#[tokio::test]
async fn grok_4_6_tolerates_a_none_thinking_level() {
    let Some(key) = api_key() else {
        eprintln!("skipping: OPENROUTER_API_KEY not set");
        return;
    };
    let provider = OpenRouterProvider::new(key).expect("provider builds");
    let previous = runtime_config::thinking_level();

    runtime_config::set_thinking_level(Some("none".to_string()));
    let result = provider
        .complete(request("Reply with the word ready."))
        .await;
    runtime_config::set_thinking_level(previous);

    let response = result.expect("`none` must be downgraded, not sent verbatim");
    assert!(!response.message.content.is_empty(), "expected content");
}

/// A high-effort streaming turn should emit reasoning as thinking chunks.
#[tokio::test]
async fn grok_4_6_streams_thinking_chunks_at_high_effort() {
    let Some(key) = api_key() else {
        eprintln!("skipping: OPENROUTER_API_KEY not set");
        return;
    };
    let provider = OpenRouterProvider::new(key).expect("provider builds");
    let previous = runtime_config::thinking_level();
    runtime_config::set_thinking_level(Some("high".to_string()));

    let mut stream = provider
        .complete_stream(request(
            "A farmer has 12 sheep; all but 9 run away. How many remain? Explain briefly.",
        ))
        .await
        .expect("stream starts");

    let mut thinking = String::new();
    let mut text = String::new();
    while let Some(chunk) = stream.next().await {
        match chunk {
            StreamChunk::Thinking(t) => thinking.push_str(&t),
            StreamChunk::Text(t) => text.push_str(&t),
            StreamChunk::Error(e) => panic!("stream error: {e}"),
            _ => {}
        }
    }
    runtime_config::set_thinking_level(previous);

    eprintln!(
        "thinking chars: {}, text chars: {}",
        thinking.len(),
        text.len()
    );
    assert!(!text.is_empty(), "expected visible answer text");
    assert!(
        !thinking.is_empty(),
        "expected reasoning as Thinking chunks"
    );
}
