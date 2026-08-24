use crate::approval::ApprovalStore;
use std::sync::{Arc, Barrier};

#[test]
fn concurrent_decisions_record_exactly_one_winner() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let request = store
        .create_request("bash", "execute", "bash:scope", "test")
        .expect("request");
    let barrier = Arc::new(Barrier::new(16));
    let threads = (0..16)
        .map(|index| {
            let store = store.clone();
            let id = request.id.clone();
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                if index % 2 == 0 {
                    store.approve(&id, "test", "race").map(|_| ())
                } else {
                    store.deny(&id, "test", "race").map(|_| ())
                }
            })
        })
        .collect::<Vec<_>>();

    let winners = threads
        .into_iter()
        .map(|thread| usize::from(thread.join().expect("thread").is_ok()))
        .sum::<usize>();

    assert_eq!(winners, 1);
    assert!(store.decision(&request.id).unwrap().is_some());
}
