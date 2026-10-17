//! Standard-session task result mapping regressions.
use super::map_session_result;

#[test]
fn finished_run_is_completed() {
    let (status, text, error, session) = map_session_result("done".into(), "s1".into(), false, 200);
    assert_eq!(status, "completed");
    assert_eq!(text.as_deref(), Some("done"));
    assert!(error.is_none());
    assert_eq!(session.as_deref(), Some("s1"));
}

#[test]
fn budget_exhausted_run_is_failed_with_diagnostic() {
    let (status, text, error, _) = map_session_result("partial".into(), "s1".into(), true, 50);
    assert_eq!(status, "failed");
    assert_eq!(text.as_deref(), Some("partial"));
    assert!(error.unwrap().contains("step budget (50)"));
}
