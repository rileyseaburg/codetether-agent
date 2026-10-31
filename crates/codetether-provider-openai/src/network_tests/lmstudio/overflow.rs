//! Guard coverage for oversized instructions, schemas, results and media.
use super::*;

#[tokio::test]
async fn lmstudio_rejects_large_system_tools_and_results_before_post() {
    for source in ["system", "tools", "result", "unicode"] {
        let (url, server) = serve(vec![metadata(4608)]);
        let mut input = request(MODEL);
        let text = "oversized ".repeat(2000);
        match source {
            "system" => input.messages.insert(
                0,
                Message {
                    role: Role::System,
                    content: vec![ContentPart::Text { text }],
                },
            ),
            "tools" => input.tools.push(ToolDefinition {
                name: "lookup".into(),
                description: text,
                parameters: json!({"type":"object"}),
            }),
            "result" => input.messages.push(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: "call".into(),
                    content: text,
                }],
            }),
            "unicode" => {
                input.messages[0].content = vec![ContentPart::Text {
                    text: "界".repeat(1600),
                }]
            }
            _ => unreachable!(),
        }
        let error = match provider(url).complete_stream(input).await {
            Ok(_) => panic!("oversized {source} was sent"),
            Err(error) => error,
        };
        assert!(
            error.to_string().contains("context budget exceeded"),
            "{error}"
        );
        assert_eq!(server.join().unwrap().len(), 1);
    }
}

#[tokio::test]
async fn lmstudio_does_not_confuse_ascii_bytes_with_tokens() {
    let (url, server) = serve(vec![
        metadata(32768),
        reply(200, "application/json", chat()),
    ]);
    let mut input = request(MODEL);
    input.messages[0].content = vec![ContentPart::Text {
        text: "instructions ".repeat(3000),
    }];
    provider(url).complete(input).await.unwrap();
    let calls = server.join().unwrap();
    assert_eq!(calls[1].body["max_tokens"], 9);
}
