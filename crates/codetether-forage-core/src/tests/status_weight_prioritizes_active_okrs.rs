//! Shared forage regression coverage.
use super::*;

#[test]
fn status_weight_prioritizes_active_okrs() {
    assert!(status_weight(OkrStatus::Active) > status_weight(OkrStatus::Draft));
    assert!(status_weight(OkrStatus::Draft) > status_weight(OkrStatus::OnHold));
    assert_eq!(status_weight(OkrStatus::Completed), 0.0);
}
