//! Application identity and initialization notification.
use codetether_lsp_core::types::ServerInfo;

/// Supplies the application version and its initialization log event.
/// # Implementors
/// Applications provide their own package version rather than the version of
/// this library, and retain their preferred structured logging target.
pub trait Host: Send + Sync + 'static {
    /// Version advertised to the language server in the client handshake.
    fn version() -> &'static str;
    /// Observe server identity after successful initialization.
    fn initialized(server_info: &Option<ServerInfo>);
}
