//! Shared request and async readiness fixtures.
use crate::{Request, runtime};
use serde_json::json;

pub(super) fn request(multiple: bool) -> Request {
    Request::parse(json!({"questions":[{
        "question":"Which?", "header":"Choice", "multiple":multiple,
        "options":[{"label":"One", "description":"First"}, {"label":"Two"}]
    }]}))
    .unwrap()
}

pub(super) async fn wait_pending(id: &str) {
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while runtime::pending(id).is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("question was not published");
}
