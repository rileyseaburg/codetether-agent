//! Timestamp access needed for cache eviction and activity tracking.
use std::sync::Mutex;
use std::time::Instant;
/// A client whose active use is tracked by a shared timestamp.
///
/// # Implementors
/// The shared LSP client implements this trait; identity wrappers may delegate
/// to that client so cache ownership continues to count the wrapper itself.
pub trait CacheClient: Send + Sync + 'static {
    /// Borrow the timestamp used by the manager's idle eviction policy.
    fn last_used(&self) -> &Mutex<Instant>;
}
impl CacheClient for crate::LspClient {
    fn last_used(&self) -> &Mutex<Instant> {
        crate::LspClient::last_used(self)
    }
}
