//! Device metadata for agent-controlled pages.

/// Pixel density and coarse device form-factor metadata.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::DeviceScale;
///
/// let scale = DeviceScale::new(2.0, true).unwrap();
/// assert_eq!(scale.factor, 2.0);
/// assert!(scale.is_mobile);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceScale {
    /// CSS pixel to device pixel multiplier.
    pub factor: f64,
    /// Whether the page should be treated as a mobile viewport.
    pub is_mobile: bool,
}

impl DeviceScale {
    /// Build validated device metadata.
    ///
    /// # Arguments
    ///
    /// * `factor` - CSS pixel to device pixel multiplier.
    /// * `is_mobile` - Whether mobile viewport behavior is requested.
    ///
    /// # Returns
    ///
    /// A validated device scale descriptor.
    ///
    /// # Errors
    ///
    /// Returns `Err` when `factor` is not finite or is less than or equal to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::DeviceScale;
    ///
    /// assert!(DeviceScale::new(1.0, false).is_ok());
    /// assert!(DeviceScale::new(0.0, false).is_err());
    /// ```
    pub fn new(factor: f64, is_mobile: bool) -> Result<Self, String> {
        if !factor.is_finite() || factor <= 0.0 {
            return Err("device scale factor must be finite and positive".into());
        }
        Ok(Self { factor, is_mobile })
    }
}

impl Default for DeviceScale {
    fn default() -> Self {
        Self {
            factor: 1.0,
            is_mobile: false,
        }
    }
}
