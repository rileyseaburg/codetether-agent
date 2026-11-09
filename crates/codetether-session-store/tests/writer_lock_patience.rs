//! A write outlasts a multi-second hold by another process instead of failing busy.
mod writer_support;
use codetether_session_store::coordinator;
use fs2::FileExt;
use std::time::{Duration, Instant};

#[test]
fn write_waits_out_a_six_second_foreign_hold() {
    let (_root, path) = writer_support::setup();
    let lock = std::fs::File::open(path.with_file_name("sessions.sqlite3.writer.lock")).unwrap();
    FileExt::lock_exclusive(&lock).unwrap();
    let release = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(6));
        FileExt::unlock(&lock).unwrap();
    });
    let started = Instant::now();
    coordinator::write(&path, |db| {
        db.execute("INSERT INTO writer_probe(value) VALUES (7)", [])?;
        Ok(())
    })
    .unwrap();
    assert!(started.elapsed() >= Duration::from_secs(5));
    release.join().unwrap();
    assert_eq!(writer_support::values(&path), vec![7]);
}
