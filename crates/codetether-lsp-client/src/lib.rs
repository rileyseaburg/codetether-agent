//! LSP clients initialized with their host application's identity.
//!
//! Implement [Host] and construct [LspClient] directly, or use [LspManager]
//! for shared server discovery, caching, and document routing. Protocol and
//! transport behavior remain owned by `codetether-lsp-core`.
use anyhow::Result;
use codetether_lsp_core::LspConfig;
use std::marker::PhantomData;

mod cache;
mod deref;
mod host;
mod initialize;
mod lifecycle;
mod managed;
pub use host::Host;

/// A language-server connection with host-specific initialization identity.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_lsp_client::Host>() -> anyhow::Result<()> {
/// let client = codetether_lsp_client::LspClient::<H>::for_language("rust", None).await?;
/// client.initialize().await?;
/// client.shutdown().await?;
/// # Ok(()) }
/// ```
pub struct LspClient<H: Host> {
    inner: codetether_lsp_core::client::LspClient,
    host: PhantomData<H>,
}

/// Shared server and linter manager accepting an identified client type.
pub use codetether_lsp_core::manager::LspManager;

#[cfg(test)]
mod real_server_tests;
