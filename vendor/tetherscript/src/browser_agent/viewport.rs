//! Viewport metadata for browser-agent pages.

use crate::browser::RasterImage;
use crate::browser_agent::page::BrowserPage;

#[path = "device.rs"]
mod device;
pub use device::DeviceScale;

/// Browser viewport dimensions and device metadata.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::{DeviceScale, Viewport};
///
/// let viewport = Viewport::new(390, 844, DeviceScale::new(3.0, true).unwrap());
/// assert_eq!(viewport.width, 390);
/// assert!(viewport.is_mobile);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// Viewport width in CSS pixels.
    pub width: i64,
    /// Viewport height in CSS pixels.
    pub height: i64,
    /// CSS pixel to device pixel multiplier.
    pub device_scale_factor: f64,
    /// Whether mobile viewport behavior is requested.
    pub is_mobile: bool,
}

impl Viewport {
    /// Build a viewport snapshot.
    ///
    /// # Arguments
    ///
    /// * `width` - Viewport width in CSS pixels.
    /// * `height` - Viewport height in CSS pixels.
    /// * `device` - Device scale metadata to fold into the snapshot.
    ///
    /// # Returns
    ///
    /// A copyable viewport metadata snapshot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{DeviceScale, Viewport};
    ///
    /// let viewport = Viewport::new(800, 600, DeviceScale::default());
    /// assert_eq!(viewport.height, 600);
    /// ```
    pub fn new(width: i64, height: i64, device: DeviceScale) -> Self {
        Self {
            width,
            height,
            device_scale_factor: device.factor,
            is_mobile: device.is_mobile,
        }
    }
}

impl BrowserPage {
    /// Set viewport dimensions in CSS pixels.
    ///
    /// # Arguments
    ///
    /// * `width` - New viewport width in CSS pixels.
    /// * `height` - New viewport height in CSS pixels.
    ///
    /// # Returns
    ///
    /// `Ok(())` after updating the compatibility width and height fields.
    ///
    /// # Errors
    ///
    /// Returns `Err` when either dimension is less than or equal to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://docs", "<main>Docs</main>");
    /// page.set_viewport_size(320, 640).unwrap();
    /// assert_eq!(page.viewport().width, 320);
    /// ```
    pub fn set_viewport_size(&mut self, width: i64, height: i64) -> Result<(), String> {
        if width <= 0 || height <= 0 {
            return Err("viewport width and height must be positive".into());
        }
        self.viewport_width = width;
        self.viewport_height = height;
        Ok(())
    }

    /// Return the current viewport and device metadata snapshot.
    ///
    /// # Returns
    ///
    /// A [`Viewport`] built from the page compatibility fields and device metadata.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::new(Default::default());
    /// assert_eq!(page.viewport().device_scale_factor, 1.0);
    /// ```
    pub fn viewport(&self) -> Viewport {
        Viewport::new(self.viewport_width, self.viewport_height, self.device_scale)
    }

    /// Render the page using the current viewport metadata.
    ///
    /// # Returns
    ///
    /// A deterministic raster image sized by viewport and device scale.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the underlying deterministic renderer rejects the document.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let mut page = BrowserPage::from_html("mem://render-docs", "<main>Hi</main>");
    /// page.set_viewport_size(4, 4).unwrap();
    /// assert_eq!(page.render_raster().unwrap().width, 4);
    /// ```
    pub fn render_raster(&self) -> Result<RasterImage, String> {
        let scale = self.device_scale.factor.ceil().max(1.0) as usize;
        self.session
            .render_raster(self.viewport_width, Some(self.viewport_height), scale)
    }
}
