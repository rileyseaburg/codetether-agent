//! Dedicated blocking writer ownership and bounded FIFO admission.
use super::{CoordinationError, job::Job};
use anyhow::Result;
use std::{
    path::PathBuf,
    sync::{Arc, mpsc},
};

thread_local! { static CURRENT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub(super) fn is_current() -> bool {
    CURRENT.get()
}

pub(super) struct Writer {
    pub path: PathBuf,
    sender: mpsc::SyncSender<Job>,
}

impl Writer {
    pub(super) fn spawn(path: PathBuf) -> Result<Arc<Self>> {
        let (sender, receiver) = mpsc::sync_channel(64);
        let worker = Arc::new(Self {
            path: path.clone(),
            sender,
        });
        std::thread::Builder::new()
            .name("session-sqlite-writer".into())
            .spawn(move || {
                CURRENT.set(true);
                super::drain::run(path, receiver);
            })?;
        Ok(worker)
    }

    pub(super) fn submit(&self, job: Job) -> Result<()> {
        self.sender.try_send(job).map_err(|error| match error {
            mpsc::TrySendError::Full(_) => CoordinationError::Full,
            mpsc::TrySendError::Disconnected(_) => CoordinationError::Stopped,
        })?;
        Ok(())
    }
}
