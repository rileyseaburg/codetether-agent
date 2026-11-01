//! FIFO admission with bounded waiting; dropping a waiter or lease releases capacity.
use super::config::Config;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore},
    time::{Instant, timeout},
};

pub(super) struct Gate {
    pub(super) config: Config,
    admission: Arc<Semaphore>,
    active: Arc<Semaphore>,
}
pub(super) struct Lease {
    pub(super) deadline: Instant,
    _gate: Arc<Gate>,
    _admission: OwnedSemaphorePermit,
    _active: OwnedSemaphorePermit,
}
impl Gate {
    pub(super) fn new(config: Config) -> Arc<Self> {
        Arc::new(Self {
            config,
            admission: Arc::new(Semaphore::new(config.waiting + 1)),
            active: Arc::new(Semaphore::new(1)),
        })
    }
    pub(super) async fn acquire(self: &Arc<Self>) -> Result<Lease> {
        let admitted = self
            .admission
            .clone()
            .try_acquire_owned()
            .context("LM Studio queue full; retry after an active request finishes")?;
        let started = Instant::now();
        let active = timeout(self.config.wait, self.active.clone().acquire_owned())
            .await
            .context("LM Studio queue wait timed out")??;
        tracing::debug!(
            wait_ms = started.elapsed().as_millis() as u64,
            "LM Studio queue admitted generation"
        );
        Ok(Lease {
            deadline: Instant::now() + self.config.generation,
            _gate: Arc::clone(self),
            _admission: admitted,
            _active: active,
        })
    }
}
#[cfg(test)]
mod tests;
