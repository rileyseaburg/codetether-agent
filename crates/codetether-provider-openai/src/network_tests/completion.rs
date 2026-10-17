//! MiniMax retries with the original conservative fallback request.

use super::*;
#[tokio::test]
async fn minimax_fallback_retains_request_and_usage_behavior() {
    let error = json!({"error":{"message":"invalid params, invalid chat setting (2013)","type":"bad_request_error","code":"2013"}}).to_string();
    let (url, server) = serve(vec![
        reply(400, "application/json", error),
        reply(200, "application/json", chat()),
    ]);
    let provider = OpenAIProvider::with_base_url("fixture-key".into(), url, "minimax").unwrap();
    let mut input = request("MiniMax-M2");
    input.tools.push(ToolDefinition {
        name: "lookup".into(),
        description: "fixture".into(),
        parameters: json!({"type":"object"}),
    });
    let response = provider.complete(input).await.unwrap();
    assert_eq!(response.usage.total_tokens, 5);
    assert!(matches!(&response.message.content[0], ContentPart::Text { text } if text == "ok"));
    let calls = server.join().unwrap();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].path, "/v1/chat/completions");
    assert_eq!(calls[0].body["max_tokens"], 9);
    assert!(calls[0].body.get("tools").is_some());
    assert!(calls[0].body.get("top_p").is_some());
    assert_eq!(calls[1].body["model"], "MiniMax-M2");
    assert_eq!(calls[1].body["messages"], calls[0].body["messages"]);
    assert!(calls[1].body.get("tools").is_none());
    assert!(calls[1].body.get("temperature").is_none());
    assert!(calls[1].body.get("max_tokens").is_none());
}
