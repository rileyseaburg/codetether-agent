//! Local Morph response server and request counter for integration fixtures.

use axum::{Json, Router, extract::State, routing::post};
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::{net::TcpListener, task::JoinHandle};

#[derive(Clone)]
struct MockState {
    output: String,
    requests: Arc<AtomicUsize>,
}

async fn handler(State(state): State<MockState>) -> Json<Value> {
    state.requests.fetch_add(1, Ordering::SeqCst);
    Json(json!({"choices": [{"message": {"content": state.output}}]}))
}

pub(super) struct ServerGuard(JoinHandle<()>);

impl Drop for ServerGuard {
    fn drop(&mut self) {
        self.0.abort();
    }
}

pub(super) async fn spawn(
    output: String,
) -> anyhow::Result<(String, Arc<AtomicUsize>, ServerGuard)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let requests = Arc::new(AtomicUsize::new(0));
    let app = Router::new()
        .route("/chat/completions", post(handler))
        .with_state(MockState {
            output,
            requests: Arc::clone(&requests),
        });
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    Ok((format!("http://{addr}"), requests, ServerGuard(handle)))
}
