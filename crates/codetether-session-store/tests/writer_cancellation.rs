//! Dropping an async waiter never cancels a write already admitted to the queue.
#[path = "writer_support/hold.rs"]
mod hold;
mod writer_support;
use codetether_session_store::coordinator;
use std::{
    future::Future,
    task::{Context, Waker},
};

#[tokio::test]
async fn accepted_write_survives_caller_cancellation() {
    let (_root, path) = writer_support::setup();
    let hold = hold::hold(&path);
    let mut future = Box::pin(coordinator::write_async(&path, |db| {
        db.execute("INSERT INTO writer_probe(value) VALUES (42)", [])?;
        Ok(())
    }));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    drop(future);
    drop(hold);
    coordinator::write_async(&path, |_| Ok(())).await.unwrap();
    assert_eq!(writer_support::values(&path), vec![42]);
}

#[tokio::test]
async fn unpolled_write_never_enters_the_queue() {
    let (_root, path) = writer_support::setup();
    drop(coordinator::write_async(&path, |db| {
        db.execute("INSERT INTO writer_probe(value) VALUES (42)", [])?;
        Ok(())
    }));
    coordinator::write_async(&path, |_| Ok(())).await.unwrap();
    assert!(writer_support::values(&path).is_empty());
}
