//! One bounded acquisition queue for all desktop requests in this parent process.
use super::{failure::Failure, process::Process};
use std::time::Duration;
use tokio::{
    sync::{Mutex, MutexGuard},
    time::timeout,
};

static WORKER: Mutex<Option<Process>> = Mutex::const_new(None);

pub(super) async fn acquire(
    deadline: Duration,
) -> Result<MutexGuard<'static, Option<Process>>, Failure> {
    timeout(deadline, WORKER.lock())
        .await
        .map_err(|_| Failure::QueueTimeout)
}
