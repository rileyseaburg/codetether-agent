//! Preserve results while making failure diagnostics visible to subscribers.

use super::{super::outcome, request};

#[test]
fn failure_diagnostic_takes_precedence_over_partial_result() {
    let mut req = request("failed");
    req.result = Some("partial work".into());
    req.error = Some("tool crashed".into());
    assert_eq!(
        outcome::from_request(&req).message.as_deref(),
        Some("Error: tool crashed")
    );
    req.error = None;
    assert_eq!(
        outcome::from_request(&req).message.as_deref(),
        Some("partial work")
    );
}

#[test]
fn success_retains_the_result() {
    let mut req = request("success");
    req.result = Some("done".into());
    req.error = Some("nonfatal diagnostic".into());
    assert_eq!(outcome::from_request(&req).message.as_deref(), Some("done"));
}
