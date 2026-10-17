use super::*;
#[tokio::test]
async fn incomplete_attempts_stop_after_completion() {
    let attempt = Attempt::new(Some(2), false);
    let (result, retries) = run(&attempt, "task", 5, 0, 0).await;
    assert_eq!(result.unwrap().3, AgentLoopExit::Completed);
    assert_eq!(retries, 2);
    assert_eq!(attempt.count.load(Ordering::SeqCst), 3);
}
