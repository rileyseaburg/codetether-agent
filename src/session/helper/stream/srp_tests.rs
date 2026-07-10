//! Unit tests for SRP stop classification, fault triage, and backoff.

use super::fault::is_transient;
use super::outcome::StreamStop;
use super::restart::RestartPolicy;
use std::time::Duration;

#[test]
fn cold_stall_and_transient_fault_are_restart_eligible() {
    assert!(StreamStop::ColdStall.restart_eligible());
    assert!(
        StreamStop::Fault {
            transient: true,
            message: String::new()
        }
        .restart_eligible()
    );
}

#[test]
fn premature_end_is_restart_eligible_and_overrides_committed() {
    // A byte stream that closes before `Done` is retryable even when partial
    // content was committed: re-requesting yields one complete answer.
    assert!(StreamStop::PrematureEnd.restart_eligible());
    assert!(StreamStop::PrematureEnd.restart_over_committed());
}

#[test]
fn only_premature_end_overrides_committed() {
    // Idle mid-stream stalls keep their committed partial instead of restarting.
    assert!(!StreamStop::MidStreamStall.restart_over_committed());
    assert!(!StreamStop::ColdStall.restart_over_committed());
    assert!(!StreamStop::Clean.restart_over_committed());
    assert!(
        !StreamStop::Fault {
            transient: true,
            message: String::new()
        }
        .restart_over_committed()
    );
}

#[test]
fn clean_partial_and_permanent_fault_are_not_eligible() {
    assert!(!StreamStop::Clean.restart_eligible());
    assert!(!StreamStop::MidStreamStall.restart_eligible());
    assert!(
        !StreamStop::Fault {
            transient: false,
            message: String::new()
        }
        .restart_eligible()
    );
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
