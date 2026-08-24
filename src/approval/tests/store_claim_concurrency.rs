use crate::approval::ApprovalStore;
use std::sync::{Arc, Barrier};

#[test]
fn concurrent_claims_execute_exactly_once() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("exec_command", "execute", "scope", "test")
        .expect("request");
    store
        .approve(&request.id, "test", "approved once")
        .expect("approve");
    let barrier = Arc::new(Barrier::new(8));
    let mut threads = Vec::new();
    for worker in 0..8 {
        let store = store.clone();
        let id = request.id.clone();
        let barrier = Arc::clone(&barrier);
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            store
                .claim(
                    &id,
                    "exec_command",
                    "execute",
                    "scope",
                    &format!("worker-{worker}"),
                )
                .is_ok()
        }));
    }
    let winners = threads
        .into_iter()
        .map(|thread| usize::from(thread.join().expect("worker")))
        .sum::<usize>();
    assert_eq!(winners, 1);
}
