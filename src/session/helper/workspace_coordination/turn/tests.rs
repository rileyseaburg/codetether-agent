use super::LeaseTurn;

#[tokio::test]
async fn drop_aborts_renewal_task() {
    let renewal = tokio::spawn(std::future::pending::<()>());
    let abort = renewal.abort_handle();
    drop(LeaseTurn {
        owner: None,
        renewal: Some(renewal),
    });
    tokio::task::yield_now().await;
    assert!(abort.is_finished());
}
