//! manager default operations for language server clients.

use super::*;

impl Default for LspManager {
    fn default() -> Self {
        Self::new(None)
    }
}
