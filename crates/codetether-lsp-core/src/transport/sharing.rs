//! Lifetime-bound sharing of this transport's existing diagnostic cache.
use super::LspTransport;
use codetether_diagnostics_bridge::Publisher;
use std::sync::{Arc, Mutex};
mod pump;
#[cfg(test)]
mod tests;

pub(super) struct Sharing {
    publisher: Arc<Mutex<Option<Publisher>>>,
    task: tokio::task::AbortHandle,
}

impl Sharing {
    fn start(transport: &LspTransport, publisher: Publisher) -> Self {
        Self::from_cache(&transport.diagnostics, &transport.reader_alive, publisher)
    }

    fn from_cache(
        cache: &Arc<tokio::sync::RwLock<codetether_diagnostics_bridge::Diagnostics>>,
        alive: &Arc<std::sync::atomic::AtomicBool>,
        publisher: Publisher,
    ) -> Self {
        let publisher = Arc::new(Mutex::new(Some(publisher)));
        let task = pump::start(
            Arc::downgrade(cache),
            Arc::downgrade(alive),
            Arc::clone(&publisher),
        );
        Self {
            publisher,
            task: task.abort_handle(),
        }
    }
}

impl Drop for Sharing {
    fn drop(&mut self) {
        self.task.abort();
        // Synchronous cleanup cannot race a publication: both hold this lock.
        self.publisher
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take();
    }
}

impl LspTransport {
    pub(crate) fn start_diagnostic_sharing(&self, root_uri: &str) {
        let mut sharing = self
            .sharing
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if sharing.is_some() {
            return;
        }
        match Publisher::new(root_uri, &self.command) {
            Ok(publisher) => *sharing = Some(Sharing::start(self, publisher)),
            Err(error) => {
                tracing::warn!(server = %self.command, error = %error, "Diagnostic sharing unavailable")
            }
        }
    }

    pub(crate) fn stop_diagnostic_sharing(&self) {
        self.sharing
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take();
    }
}
