//! Groq chat uses the supported token field, stop sequences and function tools.
use super::*;

#[tokio::test]
async fn groq_completion_preserves_model_tools_and_supported_settings() {
    let (url, server) = serve(vec![
        groq_limits::catalog("openai/gpt-oss-120b", json!(8192)),
        reply(200, "application/json", chat()),
    ]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "groq").unwrap();
    let mut input = request("openai/gpt-oss-120b");
    input.stop = vec!["END".into()];
    input.tools.push(ToolDefinition {
        name: "lookup".into(),
        description: "fixture".into(),
        parameters: json!({"type":"object","properties":{}}),
    });
    let response = provider.complete(input).await.unwrap();
    assert_eq!(response.usage.total_tokens, 5);
    assert!(matches!(&response.message.content[0], ContentPart::Text { text } if text == "ok"));
    let calls = server.join().unwrap();
    let call = &calls[1];
    assert_eq!(call.path, "/v1/chat/completions");
    assert!(
        call.headers
            .to_ascii_lowercase()
            .contains("authorization: bearer fixture-key")
    );
    assert_eq!(call.body["model"], "openai/gpt-oss-120b");
    assert_eq!(call.body["max_completion_tokens"], 9);
    assert_eq!(call.body["stop"], json!(["END"]));
    assert_eq!(call.body["tools"][0]["function"]["name"], "lookup");
    assert!(call.body.get("temperature").is_some());
    assert!(call.body.get("top_p").is_some());
    for unsupported in [
        "max_tokens",
        "n",
        "metadata",
        "store",
        "logprobs",
        "frequency_penalty",
        "presence_penalty",
    ] {
        assert!(
            call.body.get(unsupported).is_none(),
            "unexpected field {unsupported}"
        );
    }
}
