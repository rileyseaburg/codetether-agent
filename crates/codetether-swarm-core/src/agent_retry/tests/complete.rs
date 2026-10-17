use super::*;
#[tokio::test]
async fn immediate_completion_needs_no_retry() {
    let attempt = Attempt::new(Some(0), false);
    let (result, retries) = run(&attempt, "task", 3, 0, 0).await;
    assert_eq!(result.unwrap().3, AgentLoopExit::Completed);
    assert_eq!(retries, 0);
    assert_eq!(attempt.count.load(Ordering::SeqCst), 1);
}
