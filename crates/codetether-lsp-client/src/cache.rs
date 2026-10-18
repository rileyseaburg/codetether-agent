//! Delegate shared-cache timestamps to the application client wrapper.
use crate::{Host, LspClient};
use std::sync::Mutex;
use std::time::Instant;
impl<H: Host> codetether_lsp_core::client_cache::CacheClient for LspClient<H> {
    fn last_used(&self) -> &Mutex<Instant> {
        std::ops::Deref::deref(self).last_used()
    }
}
