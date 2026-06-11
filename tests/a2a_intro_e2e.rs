//! End-to-end tests for A2A intro short-circuit and RPC bearer auth.
//!
//! Boots the real `A2AServer::router()` on a loopback port. No LLM
//! provider is configured in this test environment, so a successful
//! intro response is itself proof the canned path bypassed the LLM.

use codetether_agent::a2a::server::A2AServer;
use codetether_agent::a2a::types::{
    Message, MessageRole, MessageSendParams, Part, SendMessageResponse, TaskState,
};
use serde_json::json;

async fn boot() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let url = format!("http://{addr}");
    let router = A2AServer::new(A2AServer::default_card(&url)).router();
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    url
}

fn intro_params(text: &str, tagged: bool) -> MessageSendParams {
    let mut metadata = std::collections::HashMap::new();
    if tagged {
        metadata.insert("codetether.intro".to_string(), json!(true));
    }
    MessageSendParams {
        message: Message {
            message_id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::User,
            parts: vec![Part::Text {
                text: text.to_string(),
            }],
            context_id: None,
            task_id: None,
            metadata,
            extensions: vec![],
        },
        configuration: None,
    }
}

async fn rpc(url: &str, params: &MessageSendParams) -> serde_json::Value {
    reqwest::Client::new()
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0", "id": 1,
            "method": "message/send",
            "params": params
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[tokio::test]
async fn tagged_intro_completes_without_llm() {
    let url = boot().await;
    let body = rpc(&url, &intro_params("anything", true)).await;
    let task: SendMessageResponse =
        serde_json::from_value(body["result"].clone()).expect("task result");
    let SendMessageResponse::Task(task) = task else {
        panic!("expected task response");
    };
    assert_eq!(task.status.state, TaskState::Completed);
    let ack = format!("{:?}", task.artifacts);
    assert!(ack.contains("intro-ack"), "canned artifact expected");
}

#[tokio::test]
async fn legacy_intro_text_completes_without_llm() {
    let url = boot().await;
    let text = "Hello from peer-x (http://10.0.0.9:1). \
                I am online and available for A2A collaboration.";
    let body = rpc(&url, &intro_params(text, false)).await;
    assert_eq!(
        body["result"]["status"]["state"], "completed",
        "legacy intro must short-circuit: {body}"
    );
}
