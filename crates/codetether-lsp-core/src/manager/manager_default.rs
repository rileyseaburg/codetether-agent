//! manager default operations for language server clients.

use super::*;

impl<C: ManagedClient> Default for LspManager<C> {
    fn default() -> Self {
        Self::new(None)
    }
}
