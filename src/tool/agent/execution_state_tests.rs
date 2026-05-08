//! Concurrency regression tests for spawned child sessions.

use super::execution_state::try_start;
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn concurrent_start_for_same_agent_allows_only_one_runner() {
    let barrier = Arc::new(Barrier::new(2));
    let handles: Vec<_> = (0..2)
        .map(|_| spawn_start_attempt(Arc::clone(&barrier)))
        .collect();
    let started = handles
        .into_iter()
        .filter_map(|handle| handle.join().ok())
        .filter(|value| *value)
        .count();
    assert_eq!(started, 1);
}

fn spawn_start_attempt(barrier: Arc<Barrier>) -> thread::JoinHandle<bool> {
    thread::spawn(move || {
        barrier.wait();
        let guard = try_start("reviewer");
        let started = guard.is_some();
        thread::sleep(std::time::Duration::from_millis(20));
        started
    })
}
