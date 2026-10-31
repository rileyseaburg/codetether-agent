//! Failed operations roll back without poisoning later writes or hiding errors.
mod writer_support;
use codetether_session_store::coordinator::{self, CoordinationError};

#[test]
fn panicking_and_uncommitted_operations_roll_back_and_writer_recovers() {
    let (_root, path) = writer_support::setup();
    let error = coordinator::write::<()>(&path, |db| {
        db.execute_batch("BEGIN; INSERT INTO writer_probe(value) VALUES (1)")?;
        panic!("injected writer panic");
    })
    .unwrap_err();
    assert!(matches!(
        error.downcast_ref(),
        Some(CoordinationError::Panicked)
    ));
    let error = coordinator::write(&path, |db| {
        db.execute_batch("BEGIN; INSERT INTO writer_probe(value) VALUES (2)")?;
        Ok(())
    })
    .unwrap_err();
    assert!(matches!(
        error.downcast_ref(),
        Some(CoordinationError::Uncommitted)
    ));
    coordinator::write(&path, |db| {
        db.execute("INSERT INTO writer_probe(value) VALUES (3)", [])?;
        Ok(())
    })
    .unwrap();
    assert_eq!(writer_support::values(&path), vec![3]);
}

#[test]
fn reentrant_submission_fails_instead_of_deadlocking() {
    let (_root, path) = writer_support::setup();
    let nested = path.clone();
    let error =
        coordinator::write(&path, move |_| coordinator::write(&nested, |_| Ok(()))).unwrap_err();
    assert!(matches!(
        error.downcast_ref(),
        Some(CoordinationError::Reentrant)
    ));
}
