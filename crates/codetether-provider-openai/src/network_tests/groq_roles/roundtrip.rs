//! Exercise typed completion and raw/typed streaming through the provider trait.
use super::*;

pub(super) async fn check(provider_name: &str, model: &str, expected: &str) {
    let mut replies = Vec::new();
    if provider_name == "groq" {
        replies.push(groq_limits::catalog(model, json!(8192)));
    }
    replies.push(reply(200, "application/json", chat()));
    replies.push(reply(
        200,
        "text/event-stream",
        format!("{}data: [DONE]\n\n", frame(json!({"content":"ok"}))),
    ));
    let (url, server) = serve(replies);
    let provider = OpenAIProvider::with_base_url("fixture".into(), url, provider_name).unwrap();
    let input = history::input(model);
    let original = serde_json::to_value(&input.messages).unwrap();
    provider.complete(input.clone()).await.unwrap();
    let chunks: Vec<_> = provider
        .complete_stream(input.clone())
        .await
        .unwrap()
        .collect()
        .await;
    assert!(
        chunks
            .iter()
            .any(|c| matches!(c, StreamChunk::Text(text) if text == "ok"))
    );
    assert!(!chunks.iter().any(|c| matches!(c, StreamChunk::Error(_))));
    assert_eq!(serde_json::to_value(&input.messages).unwrap(), original);
    let calls = server.join().unwrap();
    let chats: Vec<_> = calls
        .iter()
        .filter(|c| c.path.ends_with("/chat/completions"))
        .collect();
    assert_eq!(chats.len(), 2);
    for call in chats {
        assertions::messages(&call.body, expected);
    }
}
