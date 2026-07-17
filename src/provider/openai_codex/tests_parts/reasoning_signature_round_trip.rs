#[test]
fn reasoning_signature_round_trips_into_next_responses_input() {
    let item = json!({
        "type": "reasoning",
        "id": "rs_1",
        "summary": [],
        "encrypted_content": "opaque"
    });
    let event = json!({"type": "response.output_item.done", "item": item});
    let mut chunks = Vec::new();
    OpenAiCodexProvider::parse_responses_event(
        &mut ResponsesSseParser::default(),
        &event,
        &mut chunks,
    );
    let StreamChunk::Thinking(signature) = &chunks[0] else {
        panic!("expected opaque reasoning signature");
    };
    let messages = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::Thinking {
            text: String::new(),
            signature: Some(signature.clone()),
        }],
    }];

    let input = OpenAiCodexProvider::convert_messages_to_responses_input(&messages);

    assert_eq!(input, [item]);
    assert_eq!(
        OpenAiCodexProvider::build_http_responses_body(&ws_tool_request())["include"],
        json!(["reasoning.encrypted_content"])
    );
}
