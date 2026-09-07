//! Lease maintenance shared by manager clones, including Windows user tokens.
//!
//! A weak client reference avoids keeping discarded managers alive. Dropping the
//! final monitor aborts renewal; replacing Kubernetes credentials restarts it.

mod api;
mod failure;
mod run;
mod schedule;
#[cfg(test)]
mod tests;

use parking_lot::Mutex;
use std::sync::{Arc, Weak};
use tokio::task::AbortHandle;
use vaultrs::client::VaultClient;

type Clients = parking_lot::RwLock<Option<Arc<VaultClient>>>;

#[derive(Default)]
pub(super) struct Monitor(Mutex<Option<AbortHandle>>);

impl Monitor {
    pub(super) fn start(&self, clients: Weak<Clients>) {
        let mut task = self.0.lock();
        if let Some(old) = task.take() {
            old.abort();
        }
        *task = Some(tokio::spawn(run::maintain(clients)).abort_handle());
    }
}

impl Drop for Monitor {
    fn drop(&mut self) {
        if let Some(task) = self.0.get_mut().take() {
            task.abort();
        }
    }
}
