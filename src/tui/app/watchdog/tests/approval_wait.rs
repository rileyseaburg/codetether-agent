//! Regression coverage for user-paced approval waits.

use std::time::{Duration, Instant};

use super::shared::{TIMEOUT, processing_state};
use crate::approval::LiveApprovalRequest;
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::tui::app::watchdog::detector::check_watchdog_stall;

#[test]
fn pending_approval_suppresses_watchdog_restart() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    crate::tui::app::state::approval_queue::reset();
    crate::tui::app::state::approval_queue::push(request("pending"));
    let mut state = processing_state();
    state.approval_waiting = true;
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    state.main_last_event_at = Some(Instant::now() - Duration::from_secs(120));
    assert!(check_watchdog_stall(&state, TIMEOUT).is_none());
    crate::tui::app::state::approval_queue::reset();
}

#[test]
fn stale_approval_flag_does_not_suppress_watchdog() {
    let _lock = lock_env();
    crate::tui::app::state::approval_queue::reset();
    let mut state = processing_state();
    state.approval_waiting = true;
    state.processing_started_at = Some(Instant::now() - Duration::from_secs(120));
    state.main_last_event_at = Some(Instant::now() - Duration::from_secs(120));
    assert!(check_watchdog_stall(&state, TIMEOUT).is_some());
}

fn request(id: &str) -> LiveApprovalRequest {
    LiveApprovalRequest::new(
        id.into(),
        "call".into(),
        "bash".into(),
        "execute".into(),
        "resource".into(),
        "reason".into(),
    )
}
