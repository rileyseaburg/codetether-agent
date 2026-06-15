//! Unit tests for SRP stop classification, fault triage, and backoff.

use super::fault::is_transient;
use super::outcome::StreamStop;
use super::restart::RestartPolicy;
use std::time::Duration;

#[test]
fn cold_stall_and_transient_fault_are_restart_eligible() {
    assert!(StreamStop::ColdStall.restart_eligible());
    assert!(StreamStop::Fault { transient: true }.restart_eligible());
}

#[test]
fn clean_partial_and_permanent_fault_are_not_eligible() {
    assert!(!StreamStop::Clean.restart_eligible());
    assert!(!StreamStop::MidStreamStall.restart_eligible());
    assert!(!StreamStop::Fault { transient: false }.restart_eligible());
}

#[test]
fn fault_triage_prefers_permanent_markers() {
    assert!(is_transient("connection reset by peer"));
    assert!(is_transient("HTTP 503 service unavailable"));
    assert!(!is_transient("context length exceeded"));
    assert!(!is_transient("401 unauthorized"));
    // permanent wins even when a transient-looking token is present
    assert!(!is_transient("403 forbidden after 500ms"));
}

#[test]
fn backoff_grows_exponentially() {
    let p = RestartPolicy::default();
    assert_eq!(p.backoff(0), Duration::from_secs(2));
    assert_eq!(p.backoff(1), Duration::from_secs(4));
    assert_eq!(p.backoff(2), Duration::from_secs(8));
}
