use super::AbortOnDrop;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

struct DropSignal(Arc<AtomicBool>);

impl Drop for DropSignal {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

#[tokio::test]
async fn dropping_guard_aborts_started_worker() {
    let dropped = Arc::new(AtomicBool::new(false));
    let child_signal = Arc::clone(&dropped);
    let handle = tokio::spawn(async move {
        let _signal = DropSignal(child_signal);
        std::future::pending::<()>().await;
    });
    tokio::task::yield_now().await;
    drop(AbortOnDrop::new(handle));
    for _ in 0..10 {
        if dropped.load(Ordering::Acquire) {
            return;
        }
        tokio::task::yield_now().await;
    }
    panic!("aborted worker future was not dropped");
}
