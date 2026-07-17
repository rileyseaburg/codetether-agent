#![cfg(feature = "tetherscript")]

use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};
use codetether_agent::config::{Config, ProviderConfig};
use codetether_agent::provider::{
    CompletionRequest, ContentPart, Message, Provider, ProviderRegistry, Role,
};
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

#[derive(Clone, Debug)]
struct CapturedRequest {
    auth: Option<String>,
    body: Value,
}

#[derive(Clone, Default)]
struct MockChatState {
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
}

async fn mock_chat_handler(
    State(state): State<MockChatState>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Json<Value> {
    state.requests.lock().await.push(CapturedRequest {
        auth: headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string),
        body,
    });

    Json(json!({
        "id": "chatcmpl-test",
        "object": "chat.completion",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "mocked usai ok"
            },
            "finish_reason": "stop"
        }]
    }))
}

async fn spawn_mock_usai_server() -> anyhow::Result<(
    String,
    Arc<Mutex<Vec<CapturedRequest>>>,
    tokio::task::JoinHandle<()>,
)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let state = MockChatState::default();
    let requests = state.requests.clone();
    let app = Router::new()
        .route("/api/v1/chat/completions", post(mock_chat_handler))
        .with_state(state);
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    Ok((format!("http://{addr}/api/v1"), requests, handle))
}

#[tokio::test]
async fn usai_tetherscript_models_include_expected_model_info_fields() {
    let provider = codetether_agent::provider::tetherscript_provider::usai::new("test-key", None)
        .expect("usai provider should initialize");

    let models = provider
        .list_models()
        .await
        .expect("usai list_models should deserialize into ModelInfo");

    assert!(models.iter().any(|model| model.id == "gemini-2.5-flash"));
    assert!(models.iter().any(|model| model.id == "claude_4_5_sonnet"));

    for model in models {
        assert_eq!(model.provider, "usai");
        assert!(model.context_window > 0);
        assert_eq!(model.max_output_tokens, Some(8192));
        assert!(model.supports_streaming);
    }
}

#[test]
fn usai_default_base_url_targets_gsa_production_api() {
    assert_eq!(
        codetether_agent::provider::tetherscript_provider::usai::default_base_url(),
        "https://api.gsa.usai.gov/api/v1"
    );
}

#[tokio::test]
async fn from_config_registers_usai_provider() {
    let mut config = Config::default();
    config.providers.insert(
        "usai".to_string(),
        ProviderConfig {
            api_key: Some("test-key".to_string()),
            base_url: Some("http://localhost:8080/api/v1".to_string()),
            headers: HashMap::new(),
            organization: None,
        },
    );

    let registry = ProviderRegistry::from_config(&config)
        .await
        .expect("config registry should initialize");

    assert!(registry.list().contains(&"usai"));
    let provider = registry
        .get("usai")
        .expect("usai provider should be registered");
    let models = provider
        .list_models()
        .await
        .expect("registered usai provider should list models");
    assert!(models.iter().any(|model| model.id == "gemini-2.5-flash"));
}

#[tokio::test]
async fn usai_aliases_resolve_to_canonical_provider() {
    let mut config = Config::default();
    config.providers.insert(
        "gsai".to_string(),
        ProviderConfig {
            api_key: Some("test-key".to_string()),
            base_url: Some("http://localhost:8080/api/v1".to_string()),
            headers: HashMap::new(),
            organization: None,
        },
    );

    let registry = ProviderRegistry::from_config(&config)
        .await
        .expect("config registry should initialize");

    assert!(registry.list().contains(&"usai"));
    assert!(registry.resolve_model("gsai/gemini-2.5-flash").is_ok());
    assert!(registry.resolve_model("usai-gov/gemini-2.5-flash").is_ok());
}

#[tokio::test]
async fn usai_completion_posts_openai_compatible_payload_to_configured_base_url() {
    let (base_url, requests, handle) = spawn_mock_usai_server()
        .await
        .expect("mock USAi server should start");
    let provider =
        codetether_agent::provider::tetherscript_provider::usai::new("test-key", Some(&base_url))
            .expect("usai provider should initialize");

    let response = provider
        .complete(CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "reply with usai ok".to_string(),
                }],
            }],
            tools: vec![],
            model: "gemini-2.5-flash".to_string(),
            temperature: Some(0.2),
            top_p: None,
            max_tokens: None,
            stop: vec![],
        })
        .await
        .expect("mocked usai completion should succeed");

    let text = response
        .message
        .content
        .iter()
        .find_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .expect("assistant response should include text");
    assert_eq!(text, "mocked usai ok");

    let captured = requests.lock().await;
    assert_eq!(captured.len(), 1);
    assert_eq!(captured[0].auth.as_deref(), Some("Bearer test-key"));
    assert_eq!(captured[0].body["model"], "gemini-2.5-flash");
    let temperature = captured[0].body["temperature"]
        .as_f64()
        .expect("temperature should be numeric");
    assert!((temperature - 0.2).abs() < 0.000_001);
    assert_eq!(captured[0].body["messages"][0]["role"], "user");
    assert_eq!(
        captured[0].body["messages"][0]["content"],
        "reply with usai ok"
    );

    handle.abort();
}
