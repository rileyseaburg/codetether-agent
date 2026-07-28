//! Browser page accessors for media emulation metadata.

use super::{ColorScheme, ForcedColors, MediaEmulation, ReducedMotion};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return the current media emulation snapshot.
    ///
    /// # Returns
    ///
    /// A copy of the page's media emulation metadata.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, ColorScheme};
    ///
    /// let page = BrowserPage::new(Default::default());
    /// assert_eq!(page.media().color_scheme, ColorScheme::NoPreference);
    /// ```
    pub fn media(&self) -> MediaEmulation {
        self.media
    }

    /// Replace all media emulation metadata.
    ///
    /// # Arguments
    ///
    /// * `media` - Complete media emulation snapshot to install.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, ColorScheme, MediaEmulation};
    ///
    /// let mut page = BrowserPage::new(Default::default());
    /// page.set_media_emulation(MediaEmulation {
    ///     color_scheme: ColorScheme::Dark,
    ///     ..Default::default()
    /// });
    /// assert_eq!(page.media().color_scheme, ColorScheme::Dark);
    /// ```
    pub fn set_media_emulation(&mut self, media: MediaEmulation) {
        self.media = media;
    }

    /// Set the emulated color-scheme preference.
    ///
    /// # Arguments
    ///
    /// * `color_scheme` - Color-scheme preference to expose to page logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, ColorScheme};
    ///
    /// let mut page = BrowserPage::new(Default::default());
    /// page.set_color_scheme(ColorScheme::Dark);
    /// assert_eq!(page.media().color_scheme, ColorScheme::Dark);
    /// ```
    pub fn set_color_scheme(&mut self, color_scheme: ColorScheme) {
        self.media.color_scheme = color_scheme;
    }

    /// Set the emulated reduced-motion preference.
    ///
    /// # Arguments
    ///
    /// * `reduced_motion` - Motion preference to expose to page logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, ReducedMotion};
    ///
    /// let mut page = BrowserPage::new(Default::default());
    /// page.set_reduced_motion(ReducedMotion::Reduce);
    /// assert_eq!(page.media().reduced_motion, ReducedMotion::Reduce);
    /// ```
    pub fn set_reduced_motion(&mut self, reduced_motion: ReducedMotion) {
        self.media.reduced_motion = reduced_motion;
    }

    /// Set the emulated forced-colors state.
    ///
    /// # Arguments
    ///
    /// * `forced_colors` - Forced-colors state to expose to page logic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, ForcedColors};
    ///
    /// let mut page = BrowserPage::new(Default::default());
    /// page.set_forced_colors(ForcedColors::Active);
    /// assert_eq!(page.media().forced_colors, ForcedColors::Active);
    /// ```
    pub fn set_forced_colors(&mut self, forced_colors: ForcedColors) {
        self.media.forced_colors = forced_colors;
    }
}
