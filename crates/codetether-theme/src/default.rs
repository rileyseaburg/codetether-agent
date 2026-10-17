//! default for portable theme configuration.

use crate::Theme;

impl Default for Theme {
    /// Use the marketing preset by default.
    fn default() -> Self {
        Self::marketing()
    }
}
