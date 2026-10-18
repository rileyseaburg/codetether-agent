//! The default palette retains the marketing preset.
use super::ColorPalette;

impl Default for ColorPalette {
    fn default() -> Self {
        Self::marketing()
    }
}
