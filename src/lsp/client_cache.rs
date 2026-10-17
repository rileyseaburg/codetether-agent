//! Delegate shared-cache timestamps to the application client wrapper.
use super::client::LspClient;
use std::sync::Mutex;
use std::time::Instant;
impl codetether_lsp_core::client_cache::CacheClient for LspClient {
    fn last_used(&self) -> &Mutex<Instant> {
        std::ops::Deref::deref(self).last_used()
    }
}
