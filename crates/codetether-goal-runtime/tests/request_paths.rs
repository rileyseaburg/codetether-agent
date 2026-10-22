#[path = "request_paths/fixture.rs"]
mod fixture;
#[path = "request_paths/host.rs"]
mod host;
#[path = "request_paths/provider.rs"]
mod provider;
#[path = "request_paths/refresh.rs"]
mod refresh;
#[path = "request_paths/wire.rs"]
mod wire;
use codetether_provider_core::{ContentPart, Role};

#[tokio::test]
async fn prepared_builder_uses_the_snapshot_refreshed_during_preparation() {
    use codetether_provider_core::runtime_context::append;
    let mut input = fixture::prepared(append("stable rules", "stale count"));
    input.refresh_to = Some(append("stable rules", "current count"));
    let request = codetether_context_request::attempt::build(&mut input)
        .await
        .unwrap();
    let rendered = serde_json::to_string(&request.messages).unwrap();
    assert!(rendered.contains("current count"));
    assert!(!rendered.contains("stale count"));
    assert!(
        matches!(&request.messages[0].content[0], ContentPart::Text {text} if text == "stable rules")
    );
}

#[tokio::test]
async fn prepared_foreground_and_worker_requests_keep_accounting_out_of_policy() {
    let (first, second) = fixture::consecutive().await.unwrap();
    assert_eq!(
        serde_json::to_value(&first.messages[..3]).unwrap(),
        serde_json::to_value(&second.messages[..3]).unwrap()
    );
    assert!(
        matches!(&first.messages[0].content[0], ContentPart::Text{text} if text == "stable rules")
    );
    assert!(
        matches!(&second.messages[3].content[0], ContentPart::Text{text} if text.contains("750 tokens") && text.contains("Do not deploy"))
    );
    assert_eq!(second.messages[3].role, Role::User);
    assert_eq!(first.messages.len(), 6);
    assert!(
        matches!(&second.messages[4].content[0], ContentPart::Text{text} if text == "Optional recalled evidence")
    );
    assert!(
        matches!(&second.messages[5].content[0], ContentPart::Text{text} if text == "continue")
    );
}

#[tokio::test]
async fn huge_runtime_state_is_mandatory_and_enters_budget_recovery() {
    let mut input = fixture::prepared(codetether_provider_core::runtime_context::append(
        "rules",
        &"required ".repeat(200_000),
    ));
    input.model = "mistral".into();
    let error = codetether_context_request::attempt::build(&mut input)
        .await
        .unwrap_err();
    assert!(codetether_context_policy::request::recovery::is_prompt_too_long_error(&error));
}

#[tokio::test]
async fn a_prepared_model_switch_removes_only_incompatible_reasoning() {
    let prompt = codetether_provider_core::runtime_context::append("stable rules", "Do not deploy");
    let mut input = fixture::prepared(prompt);
    input.messages[1].content.push(ContentPart::Thinking {
        text: String::new(),
        signature: Some(
            "codetether:openai-reasoning:{\"type\":\"reasoning\",\"encrypted_content\":\"opaque\"}"
                .into(),
        ),
    });
    let codex = codetether_context_request::attempt::build(&mut input)
        .await
        .unwrap();
    input.provider = "anthropic".into();
    input.model = "claude-sonnet-4-5".into();
    let switched = codetether_context_request::attempt::build(&mut input)
        .await
        .unwrap();
    assert_eq!(codex.messages[2].content.len(), 2);
    assert_eq!(switched.messages[2].content.len(), 1);
    assert_eq!(
        serde_json::to_value(&codex.messages[..2]).unwrap(),
        serde_json::to_value(&switched.messages[..2]).unwrap()
    );
    assert!(
        matches!(&switched.messages[3].content[0], ContentPart::Text { text } if text.contains("Do not deploy"))
    );
    assert_eq!(
        input.messages[1].content.len(),
        2,
        "source history is untouched"
    );
}

#[tokio::test]
async fn live_tool_rounds_remain_entirely_before_changing_runtime_data() {
    let mut input = fixture::prepared(codetether_provider_core::runtime_context::append(
        "stable rules",
        "tokens: 1",
    ));
    input.messages.pop();
    input.messages.push(codetether_provider_core::Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call".into(),
            name: "read".into(),
            arguments: "{}".into(),
            thought_signature: None,
        }],
    });
    input.messages.push(codetether_provider_core::Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call".into(),
            content: "verified result".into(),
        }],
    });
    let history = input.messages.clone();
    let (first, _) = fixture::build_both(input.clone()).await.unwrap();
    input.prompt = codetether_provider_core::runtime_context::append("stable rules", "tokens: 750");
    let (second, _) = fixture::build_both(input).await.unwrap();
    let prefix = history.len() + 1;
    assert_eq!(
        serde_json::to_value(&first.messages[..prefix]).unwrap(),
        serde_json::to_value(&second.messages[..prefix]).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&first.messages[1..prefix]).unwrap(),
        serde_json::to_value(&history).unwrap()
    );
    assert!(matches!(second.messages[prefix - 1].role, Role::Tool));
    assert!(
        second
            .messages
            .last()
            .unwrap()
            .content
            .iter()
            .any(|part| matches!(part, ContentPart::Text {text} if text.contains("tokens: 750")))
    );
}
