//! Disposable localhost Vault HTTP fixture with request counters.
use super::{
    counts::Counts,
    handlers::{lookup, renew},
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub(super) struct Fixture {
    pub address: String,
    pub state: Arc<Counts>,
    task: tokio::task::AbortHandle,
}
impl Fixture {
    pub async fn new(renewable: bool, ttl: u64, lookup_denied: bool, renew_status: u16) -> Self {
        let state = Arc::new(Counts::new(renewable, ttl, lookup_denied, renew_status));
        let router = Router::new()
            .route("/v1/auth/token/lookup-self", get(lookup))
            .route("/v1/auth/token/renew-self", post(renew))
            .with_state(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let task = tokio::spawn(async {
            axum::serve(listener, router).await.unwrap();
        })
        .abort_handle();
        Self {
            address,
            state,
            task,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}
