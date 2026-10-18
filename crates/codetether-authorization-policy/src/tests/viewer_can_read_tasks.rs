//! Existing authorization regression case.
use super::*;

#[test]
fn viewer_can_read_tasks() {
    assert!(evaluate_local(&test_viewer(), "tasks:read"));
}
