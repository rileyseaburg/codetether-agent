//! Completing the pending slot cannot submit a second answer or replace it early.
use crate::{Action, Error, Request, runtime};
use serde_json::json;

fn request() -> Request {
    Request::parse(json!({"question":"Pick", "options":["A","B"]})).unwrap()
}

#[tokio::test]
async fn completion_closes_frontend_access_until_waiter_cleanup() {
    let id = "completion-slot-boundary";
    let task = tokio::spawn(runtime::scope(id.into(), runtime::ask(request(), None)));
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while runtime::pending(id).is_none() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(runtime::act(id, Action::Submit));
    assert!(!runtime::act(id, Action::Submit));
    assert!(!runtime::dismiss(id));
    assert!(runtime::pending(id).is_none());
    let duplicate = runtime::scope(id.into(), runtime::ask(request(), None)).await;
    assert!(matches!(duplicate, Err(Error::Busy)));
    assert_eq!(task.await.unwrap().unwrap(), vec![vec!["A"]]);
    assert!(runtime::pending(id).is_none());
}
