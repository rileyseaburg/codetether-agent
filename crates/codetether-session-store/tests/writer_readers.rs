//! WAL readers can observe committed data while a queued write is active.
mod writer_support;
use codetether_session_store::coordinator;
use std::{sync::mpsc, time::Duration};

#[test]
fn reader_remains_available_until_commit_acknowledgement() {
    let (_root, path) = writer_support::setup();
    let writer_path = path.clone();
    let (started, ready) = mpsc::channel();
    let (release, wait) = mpsc::channel();
    let writer = std::thread::spawn(move || {
        coordinator::write(&writer_path, move |db| {
            let tx = db.transaction()?;
            tx.execute("INSERT INTO writer_probe(value) VALUES (42)", [])?;
            started.send(())?;
            wait.recv_timeout(Duration::from_secs(10))?;
            tx.commit()?;
            Ok(())
        })
        .unwrap();
    });
    ready.recv_timeout(Duration::from_secs(10)).unwrap();
    assert!(writer_support::values(&path).is_empty());
    let (_other_root, other_path) = writer_support::setup();
    assert!(writer_support::values(&other_path).is_empty());
    release.send(()).unwrap();
    writer.join().unwrap();
    assert_eq!(writer_support::values(&path), vec![42]);
}
