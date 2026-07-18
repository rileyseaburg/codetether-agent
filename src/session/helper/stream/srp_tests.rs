//! Unit tests for SRP stop classification.

use super::outcome::StreamStop;

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
fn incomplete_stops_are_restart_eligible_after_content() {
    assert!(StreamStop::MidStreamStall.restart_eligible());
    assert!(StreamStop::MidStreamStall.restart_over_committed());
    assert!(StreamStop::PrematureEnd.restart_eligible());
    assert!(StreamStop::PrematureEnd.restart_over_committed());
}

#[test]
fn transient_fault_overrides_content_but_complete_or_cold_stops_do_not() {
    assert!(!StreamStop::ColdStall.restart_over_committed());
    assert!(!StreamStop::Clean.restart_over_committed());
    assert!(
        StreamStop::Fault {
            transient: true,
            message: String::new()
        }
        .restart_over_committed()
    );
}

#[test]
fn clean_and_permanent_fault_are_not_eligible() {
    assert!(!StreamStop::Clean.restart_eligible());
    assert!(
        !StreamStop::Fault {
            transient: false,
            message: String::new()
        }
        .restart_eligible()
    );
}
