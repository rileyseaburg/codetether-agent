use super::execution_state::{abort, register, try_start};

#[tokio::test]
async fn abort_cancels_registered_turn() {
    let guard = try_start("abort-test-agent").expect("agent should start");
    let handle = tokio::spawn(std::future::pending::<()>());
    register("abort-test-agent", &handle);
    assert!(abort("abort-test-agent"));
    assert!(handle.await.unwrap_err().is_cancelled());
    drop(guard);
}
