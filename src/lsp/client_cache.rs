//! Cache specialization for the application identity wrapper.
use super::client::LspClient;
use std::sync::{Arc, Mutex};
use std::time::Instant;
pub(super) type ClientCache = codetether_lsp_core::client_cache::ClientCache<LspClient>;
impl codetether_lsp_core::client_cache::CacheClient for LspClient {
    fn last_used(&self) -> &Mutex<Instant> {
        std::ops::Deref::deref(self).last_used()
    }
}
impl LspClient {
    pub(super) fn touch(self: &Arc<Self>) -> Arc<Self> {
        codetether_lsp_core::client_cache::touch(self)
    }
}
