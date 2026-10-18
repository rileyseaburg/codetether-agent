//! Caller-supplied authorized subprocess connection setup.
use anyhow::Result;
use codetether_mcp_core::registry::RegistryClient;
use std::{future::Future, sync::Arc};
/// Client operations plus caller-owned subprocess connection authorization.
/// # Implementors
/// Native MCP clients implement this alongside RegistryClient.
pub trait ConnectionClient: RegistryClient + Sized {
    /// Connect using the caller's existing subprocess policy.
    /// # Errors
    /// Returns authorization, process startup or initialization failures.
    fn connect_subprocess(
        command: &str,
        args: &[&str],
    ) -> impl Future<Output = Result<Arc<Self>>> + Send;
}
