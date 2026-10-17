//! Disposable localhost Vault service.
use super::{handler::handle, state::Calls};
use axum::Router;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub struct Fixture {
    pub address: String,
    pub calls: Arc<Mutex<Calls>>,
    task: tokio::task::AbortHandle,
}
impl Fixture {
    pub async fn new(replies: Vec<(u16, Value)>) -> Self {
        let calls = Arc::new(Mutex::new(Calls {
            replies: replies.into(),
            ..Default::default()
        }));
        let router = Router::new().fallback(handle).with_state(calls.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let task =
            tokio::spawn(async { axum::serve(listener, router).await.unwrap() }).abort_handle();
        Self {
            address,
            calls,
            task,
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}
