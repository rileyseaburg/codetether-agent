//! Dedicated blocking writer ownership and bounded FIFO admission.
use super::{CoordinationError, job::Job};
use anyhow::Result;
use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool, mpsc},
};

thread_local! { static CURRENT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub(super) fn is_current() -> bool {
    CURRENT.get()
}

pub(super) struct Writer {
    pub path: PathBuf,
    pub ready: Arc<AtomicBool>,
    sender: mpsc::SyncSender<Job>,
}

impl Writer {
    pub(super) fn spawn(path: PathBuf) -> Result<Arc<Self>> {
        let (sender, receiver) = mpsc::sync_channel(64);
        let ready = Arc::new(AtomicBool::new(false));
        let worker = Arc::new(Self {
            path: path.clone(),
            ready: Arc::clone(&ready),
            sender,
        });
        std::thread::Builder::new()
            .name("session-sqlite-writer".into())
            .spawn(move || {
                CURRENT.set(true);
                super::drain::run(path, ready, receiver);
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
