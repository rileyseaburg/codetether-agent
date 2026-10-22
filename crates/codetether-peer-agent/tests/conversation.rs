//! Network-to-model coverage for detached replies and conversational follow-ups.
use axum::{Json, Router, extract::State, routing::post};
use codetether_agent_messages::notice::{Host, SubAgentWatch};
use codetether_bus_core::{AgentBus, BusMessage};
use codetether_provider_core::Message;
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Default)]
struct Parent {
    id: String,
    messages: Vec<Message>,
}
impl Host for Parent {
    fn session_id(&self) -> &str {
        &self.id
    }
    fn history(&self) -> &[Message] {
        &self.messages
    }
    fn add_delegated_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

async fn reply(
    State(requests): State<Arc<Mutex<Vec<Value>>>>,
    Json(request): Json<Value>,
) -> Json<Value> {
    requests.lock().await.push(request.clone());
    Json(json!({"jsonrpc":"2.0", "id":request["id"], "result": {
        "messageId":"response", "role":"agent", "parts":[{"kind":"text", "text":"I disagree: check the return path."}]
    }}))
}

#[tokio::test]
async fn detached_reply_reaches_parent_model_and_followups_keep_context() {
    let bus = AgentBus::new().into_arc();
    codetether_bus_core::global::set_global(bus.clone());
    let requests = Arc::new(Mutex::new(Vec::new()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let app = Router::new()
        .route("/", post(reply))
        .with_state(requests.clone());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let name = format!("peer-{}", uuid::Uuid::new_v4());
    let card = serde_json::from_value(json!({
        "name":name, "description":"test", "url":endpoint, "version":"1",
        "capabilities":{}, "skills":[]
    }))
    .unwrap();
    codetether_a2a_discovery::routes::register(&card, &endpoint, None);
    let mut parent = Parent {
        id: uuid::Uuid::new_v4().to_string(),
        ..Parent::default()
    };
    let mut other = Parent {
        id: uuid::Uuid::new_v4().to_string(),
        ..Parent::default()
    };
    let mut watch = SubAgentWatch::default();
    let dispatched = codetether_peer_agent::message_or_missing(
        &name,
        "Please review",
        None,
        Some(&parent.id),
        true,
    )
    .await
    .unwrap();
    assert!(dispatched.success);
    tokio::time::timeout(Duration::from_secs(5), async {
        while parent.messages.is_empty() {
            watch.inject(&mut parent);
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the initiating model should receive the detached reply");
    let text = serde_json::to_string(&parent.messages).unwrap();
    assert!(text.contains("I disagree: check the return path."));
    watch.inject(&mut parent);
    assert_eq!(parent.messages.len(), 1, "reply must be delivered once");
    SubAgentWatch::default().inject(&mut other);
    assert!(
        other.messages.is_empty(),
        "reply must stay in its owner session"
    );
    let pending = codetether_a2a_http::inbound_turn::begin_reply(&name);
    let callback = codetether_peer_agent::message_or_missing(
        &name,
        "I need details",
        Some("new-unrelated-thread"),
        Some(&parent.id),
        false,
    )
    .await
    .unwrap();
    assert!(!callback.success);
    assert!(
        callback
            .output
            .contains("already awaiting your A2A response")
    );
    assert_eq!(
        requests.lock().await.len(),
        1,
        "a reverse callback must not start another peer agent"
    );
    drop(pending);
    for (owner, context) in [
        (&parent.id, None),
        (&other.id, None),
        (&parent.id, Some("explicit-thread")),
    ] {
        let result = codetether_peer_agent::message_or_missing(
            &name,
            "Follow up",
            context,
            Some(owner),
            false,
        )
        .await
        .unwrap();
        assert!(result.success);
    }
    let unowned =
        codetether_peer_agent::message_or_missing(&name, "No owner to notify", None, None, true)
            .await
            .unwrap();
    assert!(
        unowned.output.contains("I disagree"),
        "unroutable detached replies must be returned directly"
    );
    let requests = requests.lock().await;
    let context = |index: usize| requests[index]["params"]["message"]["contextId"].clone();
    assert!(context(0).is_string());
    assert_eq!(context(0), context(1));
    assert_ne!(context(0), context(2));
    assert_eq!(context(3), "explicit-thread");
    let delivered = bus
        .recorder
        .recent(20, Some(&format!("agent.{}", parent.id)));
    assert_eq!(
        delivered.len(),
        1,
        "synchronous replies must not be delivered again"
    );
    assert!(
        matches!(&delivered[0].message, BusMessage::AgentMessage { from, .. } if from == &name)
    );
    codetether_a2a_discovery::routes::remove(&name);
    server.abort();
}
