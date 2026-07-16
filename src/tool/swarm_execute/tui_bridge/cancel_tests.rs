use super::super::{Observer, drain};
use super::test_support::{LOCK, task};
use crate::tui::swarm_view::SwarmViewState;

#[test]
fn dropped_direct_swarm_terminates_tui_state() {
    let _guard = LOCK.lock().unwrap();
    let mut view = SwarmViewState::default();
    drain(&mut view);
    let observer = Observer::begin(&[task("Cancelled")], 20);
    drain(&mut view);
    assert!(view.active);
    drop(observer);
    assert!(drain(&mut view));
    assert!(view.complete);
    assert!(!view.active);
    assert_eq!(
        view.error.as_deref(),
        Some("Direct swarm cancelled before completion")
    );
}
