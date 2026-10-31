//! Queue admission is bounded and accepted writes retain FIFO order.
#[path = "writer_support/hold.rs"]
mod hold;
mod writer_support;
use codetether_session_store::coordinator::{self, CoordinationError};
use std::{
    future::Future,
    task::{Context, Waker},
};

#[tokio::test]
async fn sixty_four_queued_operations_preserve_order_and_reject_overload() {
    let (_root, path) = writer_support::setup();
    let hold = hold::hold(&path);
    let mut pending = Vec::new();
    for value in 0..64 {
        let mut future = Box::pin(coordinator::write_async(&path, move |db| {
            db.execute("INSERT INTO writer_probe(value) VALUES (?1)", [value])?;
            Ok(())
        }));
        assert!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop()))
                .is_pending()
        );
        pending.push(future);
    }
    let error = coordinator::write_async(&path, |_| Ok(()))
        .await
        .unwrap_err();
    assert!(matches!(
        error.downcast_ref(),
        Some(CoordinationError::Full)
    ));
    drop(hold);
    for future in pending {
        future.await.unwrap();
    }
    assert_eq!(writer_support::values(&path), (0..64).collect::<Vec<_>>());
}
