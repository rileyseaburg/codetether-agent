//! state responsibility for the stdio LSP transport.

use super::*;

impl LspTransport {
    /// Check if the server is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Mark the server as initialized
    pub fn set_initialized(&self, value: bool) {
        self.initialized
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }
}

impl Drop for LspTransport {
    fn drop(&mut self) {
        if self.is_initialized() {
            tracing::debug!("LspTransport dropped while still initialized");
        }
    }
}
