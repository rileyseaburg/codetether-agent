use super::ChildTask;
use crate::session::Session;

#[tokio::test]
async fn drop_aborts_child_turn() {
    let handle = tokio::spawn(std::future::pending::<anyhow::Result<Session>>());
    let abort = handle.abort_handle();
    drop(ChildTask::new(handle));
    tokio::task::yield_now().await;
    assert!(abort.is_finished());
}
