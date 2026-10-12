//! Transient error recovery tests.

use super::*;

/// A transient failure can recover before exhausting the budget.
#[tokio::test(start_paused = true)]
async fn transient_rate_limit_recovers_before_budget() {
    let calls = Arc::new(AtomicU32::new(0));
    let seen = Arc::clone(&calls);
    let (body, status) = send_with_retry(|| {
        let seen = Arc::clone(&seen);
        async move {
            let attempt = seen.fetch_add(1, Ordering::SeqCst) + 1;
            if attempt < 2 {
                return Ok((
                    String::from("Too Many Requests"),
                    reqwest::StatusCode::TOO_MANY_REQUESTS,
                ));
            }
            Ok((String::from("{\"ok\":true}"), reqwest::StatusCode::OK))
        }
    })
    .await
    .expect("retry should recover");
    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body, "{\"ok\":true}");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
