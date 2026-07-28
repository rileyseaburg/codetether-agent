//! Media emulation metadata for agent-controlled pages.

#[path = "media_page.rs"]
mod media_page;

/// Emulated `prefers-color-scheme` value.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::ColorScheme;
///
/// let scheme = ColorScheme::Dark;
/// assert_eq!(scheme, ColorScheme::Dark);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScheme {
    /// Emulate a light color scheme.
    Light,
    /// Emulate a dark color scheme.
    Dark,
    /// Do not express a color-scheme preference.
    NoPreference,
}

/// Emulated `prefers-reduced-motion` value.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::ReducedMotion;
///
/// assert_eq!(ReducedMotion::default(), ReducedMotion::NoPreference);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReducedMotion {
    /// Do not express a motion preference.
    NoPreference,
    /// Prefer reduced motion.
    Reduce,
}

/// Emulated `forced-colors` value.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::ForcedColors;
///
/// assert_eq!(ForcedColors::default(), ForcedColors::None);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForcedColors {
    /// Forced colors are not active.
    None,
    /// Forced colors are active.
    Active,
}

/// Copyable media emulation snapshot for a page.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::{ColorScheme, MediaEmulation};
///
/// let media = MediaEmulation { color_scheme: ColorScheme::Dark, ..Default::default() };
/// assert_eq!(media.color_scheme, ColorScheme::Dark);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MediaEmulation {
    /// Emulated color-scheme preference.
    pub color_scheme: ColorScheme,
    /// Emulated reduced-motion preference.
    pub reduced_motion: ReducedMotion,
    /// Emulated forced-colors state.
    pub forced_colors: ForcedColors,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::NoPreference
    }
}

impl Default for ReducedMotion {
    fn default() -> Self {
        Self::NoPreference
    }
}

impl Default for ForcedColors {
    fn default() -> Self {
        Self::None
    }
}
