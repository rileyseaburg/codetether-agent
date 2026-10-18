//! Delegate server operations to the shared client.

use super::{Host, LspClient};

impl<H: Host> std::ops::Deref for LspClient<H> {
    type Target = codetether_lsp_core::client::LspClient;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
