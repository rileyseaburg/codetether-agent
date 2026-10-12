use super::from_message;
use std::time::Duration;

#[test]
fn parses_openai_rate_limit_delays() {
    assert_eq!(
        from_message("Please try again in 28ms."),
        Some(Duration::from_millis(28))
    );
    assert_eq!(
        from_message("Please try again in 1.898s."),
        Some(Duration::from_secs_f64(1.898))
    );
    assert_eq!(
        from_message("Try again in 35 seconds."),
        Some(Duration::from_secs(35))
    );
}

#[test]
fn ignores_errors_without_retry_guidance() {
    assert_eq!(from_message("service unavailable"), None);
}
