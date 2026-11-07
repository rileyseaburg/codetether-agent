//! Invalid Groq settings fail before a request reaches the network.
use super::*;

#[tokio::test]
async fn groq_rejects_unsupported_request_sizes_and_sampling_values() {
    let provider =
        OpenAIProvider::with_base_url("fixture-key".into(), "http://127.0.0.1:1/v1".into(), "groq")
            .unwrap();
    let base = request("llama-3.3-70b-versatile");
    let mut cases = Vec::new();
    let mut input = base.clone();
    input.stop = vec!["stop".into(); 5];
    cases.push(input);
    let mut input = base.clone();
    input.tools = (0..129)
        .map(|n| ToolDefinition {
            name: format!("tool_{n}"),
            description: String::new(),
            parameters: json!({}),
        })
        .collect();
    cases.push(input);
    let mut input = base.clone();
    input.max_tokens = Some(0);
    cases.push(input);
    let mut input = base.clone();
    input.temperature = Some(f32::NAN);
    cases.push(input);
    let mut input = base;
    input.top_p = Some(1.1);
    cases.push(input);
    for input in cases {
        let error = provider.complete(input.clone()).await.err().unwrap();
        assert!(error.to_string().contains("Groq"));
        let error = provider.complete_stream(input).await.err().unwrap();
        assert!(error.to_string().contains("Groq"));
    }
}
