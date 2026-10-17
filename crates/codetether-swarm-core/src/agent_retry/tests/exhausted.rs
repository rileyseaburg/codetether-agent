use super::*;
#[tokio::test]
async fn exhausted_errors_include_initial_attempt() {
    let attempt = Attempt::new(None, true);
    let (result, retries) = run(&attempt, "task", 2, 0, 0).await;
    assert_eq!(result.unwrap_err().to_string(), "provider failed");
    assert_eq!(retries, 2);
    assert_eq!(attempt.count.load(Ordering::SeqCst), 3);
}
