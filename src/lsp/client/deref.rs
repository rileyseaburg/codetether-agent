//! Delegate server operations to the shared client.

use super::LspClient;

impl std::ops::Deref for LspClient {
    type Target = codetether_lsp_core::client::LspClient;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
