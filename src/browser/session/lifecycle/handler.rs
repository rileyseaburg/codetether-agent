use chromiumoxide::handler::Handler;
use futures::StreamExt;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{sync::watch, task::JoinHandle};

struct AliveGuard(Arc<AtomicBool>);

impl Drop for AliveGuard {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

pub(super) fn spawn(handler: Handler) -> (Arc<AtomicBool>, watch::Sender<bool>, JoinHandle<()>) {
    let alive = Arc::new(AtomicBool::new(true));
    let (shutdown, mut rx) = watch::channel(false);
    let task_alive = Arc::clone(&alive);
    let task = tokio::spawn(async move {
        let _guard = AliveGuard(Arc::clone(&task_alive));
        let mut handler = handler;
        loop {
            tokio::select! {
                changed = rx.changed() => {
                    if changed.is_err() || *rx.borrow() {
                        break;
                    }
                }
                next = handler.next() => match next {
                    Some(Ok(())) => {}
                    Some(Err(error)) => {
                        tracing::warn!(error = %error, "browser handler exited");
                        break;
                    }
                    None => break,
                }
            }
        }
    });
    (alive, shutdown, task)
}
