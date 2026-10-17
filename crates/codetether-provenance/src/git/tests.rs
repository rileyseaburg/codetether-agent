//! Commit trailer regressions.

use super::commit_message_with_provenance;

use crate::{ExecutionOrigin, ExecutionProvenance};

#[test]
fn appends_codetether_trailers() {
    let mut provenance = ExecutionProvenance::for_operation("ralph", ExecutionOrigin::Ralph);
    provenance.session_id = Some("session-1".to_string());
    provenance.apply_worker_task("worker-1", "task-1");
    provenance.set_run_id("run-1");
    provenance.attempt_id = Some("run-1:attempt:2".to_string());
    let message = commit_message_with_provenance("feat: test", Some(&provenance));
    assert!(message.contains("CodeTether-Provenance-ID:"));
    assert!(message.contains("CodeTether-Session-ID: session-1"));
    assert!(message.contains("CodeTether-Task-ID: task-1"));
    assert!(message.contains("CodeTether-Run-ID: run-1"));
    assert!(message.contains("CodeTether-Attempt-ID: run-1:attempt:2"));
}
