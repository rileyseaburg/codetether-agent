//! Multi-server LSP and linter lifecycle, routing, and recovery.
//!
//! [LspManager] caches the caller-provided [ManagedClient] type, preserving its
//! initialization identity and Arc ownership.
mod contract;
use crate::types::*;
use anyhow::Result;
pub use contract::ManagedClient;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Manages primary language-server and diagnostic-only linter connections.
///
/// The client type supplies construction and initialization identity while
/// this manager owns configuration precedence, cache and lifecycle policy.
///
/// # Examples
/// ```
/// fn manager<C: codetether_lsp_core::manager::ManagedClient>() {
///     let manager = codetether_lsp_core::manager::LspManager::<C>::new(None);
/// }
/// ```
pub struct LspManager<C: ManagedClient> {
    pub(super) clients: crate::client_cache::ClientCache<C>,
    /// Linter clients keyed by linter name (e.g. "eslint", "ruff").
    /// These are only queried for diagnostics, not completions/definitions.
    linter_clients: crate::client_cache::ClientCache<C>,
    root_uri: Option<String>,
    /// User-supplied LSP settings from config.
    lsp_settings: Option<codetether_config::LspSettings>,
}

mod linter_config;
mod linter_diagnostics;
mod manager_capabilities;
mod manager_connections;
mod manager_default;
mod manager_documents;
mod manager_lifecycle;
mod manager_linters;
mod recovery;
