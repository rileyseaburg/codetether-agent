//! Twenty independent session writers sharing one existing SQLite database.
use codetether_session_store::connection;
#[path = "swarm_write_support/mod.rs"]
mod support;
use std::sync::{Arc, Barrier};

#[test]
fn twenty_sessions_commit_without_losing_records() {
    let root = tempfile::tempdir().unwrap();
    let locator = root.path().join("worker.json");
    drop(connection::open(&locator).unwrap());
    let ready = Arc::new(Barrier::new(20));
    let started = std::time::Instant::now();
    let handles: Vec<_> = (0..20)
        .map(|worker| {
            let path = locator.clone();
            let ready = Arc::clone(&ready);
            std::thread::spawn(move || {
                let id = format!("worker-{worker}");
                ready.wait();
                support::write(&path, &id);
            })
        })
        .collect();
    let failed = handles
        .into_iter()
        .map(|handle| handle.join())
        .filter(Result::is_err)
        .count();
    let counts = support::counts(&locator);
    if failed > 0 {
        let retained = root.keep();
        panic!(
            "{failed} writers failed; (finished sessions, records, commits, integrity)={counts:?}; database retained at {}",
            retained.join("sessions.sqlite3").display()
        );
    }
    assert_eq!(counts, (20, 2000, 2000, "ok".into()));
    eprintln!(
        "20 writers, 2000 durable commits, 2000 records, integrity ok; elapsed {:?}",
        started.elapsed()
    );
}
