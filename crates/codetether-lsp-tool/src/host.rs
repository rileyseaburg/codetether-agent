//! Host identity and manager-cache ownership for the LSP tool.
use super::State;
use codetether_lsp_core::manager::ManagedClient;
/// Supplies the existing application client type and one shared cache.
///
/// # Implementors
/// Hosts preserve their own client handshake identity and return a stable state reference.
pub trait Host: Send + Sync + 'static {
    /// Client wrapper used by the shared LSP manager.
    type Client: ManagedClient;
    /// Return the process-wide state for this host.
    fn state() -> &'static State<Self::Client>;
}
