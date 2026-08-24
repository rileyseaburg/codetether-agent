//! Durable and serialized approval-log mutation tests.

use crate::approval::ApprovalStore;
use std::sync::Arc;

#[test]
fn concurrent_request_and_decision_appends_remain_complete() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = Arc::new(ApprovalStore::open(dir.path()).expect("store"));
    let threads: Vec<_> = (0..12)
        .map(|index| {
            let store = Arc::clone(&store);
            std::thread::spawn(move || {
                let request = store
                    .create_request(
                        "exec_command",
                        "execute",
                        &format!("command-{index}"),
                        "concurrent",
                    )
                    .expect("request");
                store
                    .approve(&request.id, "test", "allow")
                    .expect("approve");
            })
        })
        .collect();
    for thread in threads {
        thread.join().expect("thread");
    }
    assert_eq!(store.events().expect("events").len(), 24);
}
