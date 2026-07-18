use super::first_final;
use crate::tool::agent::collaboration_runtime::thread_status::ThreadStatus;
use serde_json::json;
use std::time::Duration;
use tokio::sync::watch;

#[tokio::test]
async fn ignores_interrupted_until_a_final_status_arrives() {
    let (sender, receiver) = watch::channel(ThreadStatus::Interrupted);
    tokio::spawn(async move {
        tokio::task::yield_now().await;
        sender.send_replace(ThreadStatus::Completed(Some("done".into())));
    });
    let settled = first_final(
        vec![("child-id".into(), receiver)],
        tokio::time::Instant::now() + Duration::from_secs(1),
    )
    .await;
    assert_eq!(settled["child-id"], json!({"completed":"done"}));
}
