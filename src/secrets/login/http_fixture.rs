//! Isolated loopback Vault fixture; never contacts a real Vault.

use axum::{
    Json, Router,
    routing::{get, post},
};
use serde_json::json;

pub(super) struct Server {
    pub address: String,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}

pub(super) async fn start(policy: &str, permission: &str, reject: bool) -> Server {
    let policy = policy.to_owned();
    let permission = permission.to_owned();
    let app = Router::new()
        .route("/v1/auth/token/lookup-self", get(move |headers: axum::http::HeaderMap| async move {
            assert_eq!(headers.get("X-Vault-Token").unwrap(), "fixture-token");
            let status = if reject { axum::http::StatusCode::FORBIDDEN } else { axum::http::StatusCode::OK };
            (status, Json(json!({"data":{"policies":[policy],"ttl":3600,"renewable":true},"errors":["fixture-token"]})))
        }))
        .route("/v1/sys/capabilities-self", post(move || async move {
            Json(json!({"data": {
                "sys/auth": [permission], "sys/policies/acl": ["deny"],
                "sys/mounts": ["deny"], "auth/token/create": ["deny"]
            }}))
        }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    Server { address, task }
}
