//! Stateful local input transformations for one mux attachment.

use super::Resolver;

/// Stateful detach and paste processing for one terminal attachment.
/// # Examples
/// ```
/// let mut pipeline = codetether_mux_input::Pipeline::new();
/// assert_eq!(pipeline.filter(b"status", || None).data, b"status");
/// ```
pub struct Pipeline {
    detach: crate::detach::Detector,
    paste: Resolver,
}

impl Pipeline {
    /// Create an input pipeline with no pending detach or paste marker.
    pub fn new() -> Self {
        Self {
            detach: crate::detach::Detector::new(),
            paste: Resolver::default(),
        }
    }

    /// Filter one read, consulting the clipboard only for an eligible Ctrl+V.
    /// # Arguments
    /// `raw` supplies terminal bytes; `clipboard` reads local text or image content.
    /// # Returns
    /// Forwardable bytes and whether the detach sequence was consumed.
    /// # Examples
    /// ```
    /// let mut pipeline = codetether_mux_input::Pipeline::new();
    /// assert!(pipeline.filter(&[2, b'd'], || None).detach);
    /// ```
    pub fn filter(
        &mut self,
        raw: &[u8],
        clipboard: impl FnOnce() -> Option<String>,
    ) -> crate::detach::Filtered {
        let mut filtered = self.detach.filter(raw);
        filtered.data = self.paste.resolve(filtered.data, clipboard);
        filtered
    }
}
