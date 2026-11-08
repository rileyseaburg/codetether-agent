//! Groq discovery excludes inactive/speech models and keeps hosted model IDs.
use super::*;

#[tokio::test]
async fn groq_discovery_reads_limits_and_filters_non_chat_models() {
    let payload = json!({"data":[
        {"id":"openai/gpt-oss-120b","active":true,"context_window":131072,"max_completion_tokens":8192},
        {"id":"retired-model","active":false},
        {"id":"whisper-large-v3-turbo","active":true},
        {"id":"playai-tts","active":true},
        {"id":"meta-llama/llama-guard-4-12b","active":true},
        {"id":"openai/gpt-oss-safeguard-20b","active":true},
        {"id":"groq/compound","active":true}
    ]}).to_string();
    let (url, server) = serve(vec![reply(200, "application/json", payload)]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    let models = provider.list_models().await.unwrap();
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, "openai/gpt-oss-120b");
    assert_eq!(models[0].provider, "groq");
    assert_eq!(models[0].context_window, 131072);
    assert_eq!(models[0].max_output_tokens, Some(8192));
    assert!(models[0].supports_tools && models[0].supports_streaming);
    assert!(!models[1].supports_tools);
    let calls = server.join().unwrap();
    assert_eq!(calls[0].path, "/v1/models");
    assert!(
        calls[0]
            .headers
            .to_ascii_lowercase()
            .contains("authorization: bearer fixture-key")
    );
}
