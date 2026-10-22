use codetether_codex_protocol::{CodexProtocol, request::ResponsesWsBackend};

#[tokio::test]
async fn codex_http_and_websocket_keep_stable_keys_and_history() {
    let (first, second) = super::fixture::consecutive().await.unwrap();
    for backend in [ResponsesWsBackend::OpenAi, ResponsesWsBackend::ChatGptCodex] {
        let a = CodexProtocol::build_responses_ws_create_event_for_backend(
            &first,
            &first.model,
            None,
            None,
            backend,
        );
        let b = CodexProtocol::build_responses_ws_create_event_for_backend(
            &second,
            &second.model,
            None,
            None,
            backend,
        );
        assert_eq!(a["instructions"], "stable rules");
        assert_eq!(a["instructions"], b["instructions"]);
        assert_eq!(a["prompt_cache_key"], b["prompt_cache_key"]);
        assert_eq!(a["tools"], b["tools"]);
        assert_eq!(
            &a["input"].as_array().unwrap()[..2],
            &b["input"].as_array().unwrap()[..2]
        );
        assert_ne!(a["input"][2], b["input"][2]);
        let http =
            CodexProtocol::build_http_responses_body(&second, second.model.clone(), None, None);
        assert_eq!(http["prompt_cache_key"], b["prompt_cache_key"]);
        assert_eq!(http["input"], b["input"]);
    }
}

#[tokio::test]
async fn anthropic_and_bedrock_keep_dynamic_governance_in_user_content() {
    let (first, second) = super::fixture::consecutive().await.unwrap();
    let (a_system, a) = codetether_provider_anthropic::encode_messages(&first.messages, true);
    let (b_system, b) = codetether_provider_anthropic::encode_messages(&second.messages, true);
    assert_eq!(a_system, b_system);
    assert_eq!(a_system.unwrap()[0]["cache_control"]["type"], "ephemeral");
    assert_eq!(a[..2], b[..2]);
    assert_ne!(a[2], b[2]);
    assert_eq!(b[2]["role"], "user");
    assert!(b[2].to_string().contains("750 tokens"));
    let a = codetether_bedrock_protocol::build_converse_body(&first, "anthropic.claude-sonnet-4-5");
    let b =
        codetether_bedrock_protocol::build_converse_body(&second, "anthropic.claude-sonnet-4-5");
    assert_eq!(a["system"], b["system"]);
    assert_eq!(a["toolConfig"], b["toolConfig"]);
    assert_eq!(
        &a["messages"].as_array().unwrap()[..2],
        &b["messages"].as_array().unwrap()[..2]
    );
    assert_eq!(b["messages"][2]["role"], "user");
    assert!(b["messages"][2].to_string().contains("750 tokens"));
    assert!(!b["system"].to_string().contains("750"));
}

#[tokio::test]
async fn google_chat_encoding_preserves_prefix_across_a_model_switch() {
    let (first, mut second) = super::fixture::consecutive().await.unwrap();
    second.model = "gemini-fixture".into();
    let a = codetether_provider_google::GoogleProvider::convert_messages(&first.messages);
    let b = codetether_provider_google::GoogleProvider::convert_messages(&second.messages);
    assert_eq!(a[..3], b[..3]);
    assert_eq!(b[0]["role"], "system");
    assert_eq!(b[0]["content"], "stable rules");
    assert_eq!(b[3]["role"], "user");
    assert!(b[3]["content"].as_str().unwrap().contains("750 tokens"));
    assert_eq!(b.last().unwrap()["content"], "continue");
}
