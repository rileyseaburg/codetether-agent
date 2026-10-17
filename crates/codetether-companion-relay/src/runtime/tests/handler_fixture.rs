//! Private handler fixture with a no-network analyzer and paired registry.
use crate::{Analysis, Analyze, Relay, Shared, relay::now};
use codetether_companion_protocol::SessionInput;
use std::sync::Arc;

pub(super) fn paired() -> (Shared, String, String) {
    let analyze: Analyze = Arc::new(|_: Analysis| Box::pin(async { Ok(()) }));
    let relay = Relay::new(
        "synthetic-owner-credential-not-a-real-secret",
        analyze,
        "https://example.test",
        None,
    )
    .unwrap();
    let mut state = relay.lock();
    let created = state
        .create(
            SessionInput {
                model: "p/vision".into(),
                prompt: "Describe".into(),
                interval_seconds: 30,
            },
            now(),
        )
        .unwrap();
    let paired = state.pair(&created.code, now()).unwrap();
    drop(state);
    (relay, created.id, paired.device_token)
}

pub(super) async fn value(response: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(response.into_body(), 4096)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
