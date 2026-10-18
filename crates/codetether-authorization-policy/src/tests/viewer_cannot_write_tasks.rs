//! Existing authorization regression case.
use super::*;

#[test]
fn viewer_cannot_write_tasks() {
    assert!(!evaluate_local(&test_viewer(), "tasks:write"));
}
