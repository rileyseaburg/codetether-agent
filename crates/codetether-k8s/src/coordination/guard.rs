//! Renewing RAII guard for one acquired mutation lease.

use tokio::sync::oneshot;

use super::guard_task;
use super::repository::LeaseRepository;

pub struct LeaseGuard {
    stop: Option<oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<()>>,
}

impl LeaseGuard {
    pub fn start(repo: LeaseRepository, name: String, holder: String, ttl: i32) -> Self {
        let (stop, stopped) = oneshot::channel();
        let task = guard_task::spawn(repo, name, holder, ttl, stopped);
        Self {
            stop: Some(stop),
            task: Some(task),
        }
    }

    pub async fn release(mut self) {
        self.signal();
        if let Some(task) = self.task.take() {
            let _ = task.await;
        }
    }

    fn signal(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
    }
}

impl Drop for LeaseGuard {
    fn drop(&mut self) {
        self.signal();
    }
}
