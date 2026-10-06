//! Loopback authority fixture; no identity provider or control plane is contacted.
use axum::{
    Router,
    http::{HeaderMap, StatusCode, Uri},
    routing::post,
};
use tokio::{net::TcpListener, sync::mpsc, task::JoinHandle};

pub(super) struct Authority {
    pub url: String,
    pub requests: mpsc::UnboundedReceiver<(Uri, HeaderMap)>,
    task: JoinHandle<()>,
}

impl Drop for Authority {
    fn drop(&mut self) {
        self.task.abort();
    }
}

pub(super) async fn authority(status: StatusCode, body: &str) -> Authority {
    let body = body.to_owned();
    let (sender, requests) = mpsc::unbounded_channel();
    let app = Router::new().route(
        "/v1/agent/workspaces/{id}/session-access",
        post(move |uri: Uri, headers: HeaderMap| {
            let body = body.clone();
            let sender = sender.clone();
            async move {
                sender.send((uri, headers)).unwrap();
                (status, body)
            }
        }),
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    Authority {
        url,
        requests,
        task,
    }
}

pub(super) fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("authorization", "Bearer test-caller".parse().unwrap());
    headers
}
