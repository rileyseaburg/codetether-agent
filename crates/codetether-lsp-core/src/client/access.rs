//! Read-only access to client transport/configuration and cache timestamp coordination.
use super::*;
impl LspClient {
    /// Borrow transport state for diagnostic observers.
    pub fn transport(&self) -> &LspTransport {
        &self.transport
    }
    /// Borrow the language-server configuration.
    pub fn config(&self) -> &LspConfig {
        &self.config
    }
    /// Borrow the last-use timestamp lock for cache ownership accounting.
    pub fn last_used(&self) -> &std::sync::Mutex<std::time::Instant> {
        &self.last_used
    }
}
