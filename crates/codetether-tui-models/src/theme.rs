//! Simple serializable RGB theme settings.
use serde::{Deserialize, Serialize};

/// Color theme configuration for TUI rendering.
///
/// The theme stores a display name and RGB tuples for primary accents and
/// borders. Rendering code can convert these tuples into terminal colors while
/// keeping persisted configuration simple and serializable.
///
/// # Invariants
///
/// Each color channel is an `u8`, so values are always valid RGB channel values
/// in the range `0..=255`.
///
/// # Examples
///
/// ```
/// use codetether_tui_models::Theme;
///
/// let theme = Theme::default();
/// assert_eq!(theme.name, "Default");
/// assert_eq!(theme.primary, (0, 255, 255));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Human-readable theme name shown in settings or diagnostics.
    pub name: String,
    /// Primary accent color as an RGB tuple.
    pub primary: (u8, u8, u8),
    /// Border color as an RGB tuple.
    pub border: (u8, u8, u8),
}

impl Default for Theme {
    /// Creates the default cyan-accented TUI theme.
    ///
    /// # Returns
    ///
    /// Returns a [`Theme`] named `"Default"` with a cyan primary color and a
    /// neutral gray border color.
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            primary: (0, 255, 255),
            border: (100, 100, 100),
        }
    }
}
