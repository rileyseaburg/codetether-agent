#[test]
fn prompt_cache_key_is_stable_across_transports_and_turns() {
    let request = ws_tool_request();
    let http = OpenAiCodexProvider::build_http_responses_body(&request);
    let websocket = OpenAiCodexProvider::build_responses_ws_create_event(
        &request,
        "gpt-5.4",
        Some(ThinkingLevel::High),
        None,
    );
    let key = http["prompt_cache_key"].as_str().expect("HTTP cache key");

    assert_eq!(websocket["prompt_cache_key"].as_str(), Some(key));
    assert!(key.starts_with("codetether:"));
    assert_eq!(key.len(), 43);
    assert_ne!(key, OpenAiCodexProvider::prompt_cache_key("other workspace"));

    let mut later_turn = request.clone();
    later_turn.messages.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Now run the tests".to_string(),
        }],
    });
    let later = OpenAiCodexProvider::build_http_responses_body(&later_turn);

    assert_eq!(later["prompt_cache_key"].as_str(), Some(key));
}
