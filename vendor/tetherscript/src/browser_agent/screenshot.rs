//! Screenshot APIs for browser-agent pages.

#[path = "screenshot_image.rs"]
mod image;

use crate::browser::RasterImage;
use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve;

/// Cropped element screenshot and source document bounds.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::{BrowserPage, Locator};
///
/// let page = BrowserPage::from_html(
///     "mem://shot",
///     "<main id='box' style='width:1px;height:1px'></main>",
/// );
/// let shot = page.element_screenshot(&Locator::css("#box")).unwrap();
/// assert_eq!(shot.bounds.width, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementScreenshot {
    /// Cropped pixels for the resolved element.
    pub image: RasterImage,
    /// Element bounds in CSS pixels before device scaling.
    pub bounds: BoundingBox,
}

impl BrowserPage {
    /// Capture the current viewport as a native RGBA raster image.
    ///
    /// # Errors
    ///
    /// Returns `Err` when rasterization or viewport cropping fails.
    pub fn screenshot(&self) -> Result<RasterImage, String> {
        let scale = image::scale_factor(self.device_scale.factor);
        let source = self
            .session
            .render_raster(self.viewport_width, None, scale)?;
        image::crop_css(&source, self.viewport_capture_bounds(), scale)
    }

    /// Capture the current viewport as binary PPM bytes.
    ///
    /// # Errors
    ///
    /// Returns `Err` when [`BrowserPage::screenshot`] fails.
    pub fn screenshot_ppm(&self) -> Result<Vec<u8>, String> {
        self.screenshot().map(|image| image.to_ppm())
    }

    /// Capture a resolved element and return its cropped pixels plus bounds.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the locator is not strict or rasterization fails.
    pub fn element_screenshot(&self, locator: &Locator) -> Result<ElementScreenshot, String> {
        let resolved = resolve::resolve(&self.session, self.viewport_width, locator)?;
        let scale = image::scale_factor(self.device_scale.factor);
        let source = self
            .session
            .render_raster(self.viewport_width, None, scale)?;
        Ok(ElementScreenshot {
            image: image::crop_css(&source, resolved.bounds, scale)?,
            bounds: resolved.bounds,
        })
    }

    fn viewport_capture_bounds(&self) -> BoundingBox {
        BoundingBox {
            x: self.session.scroll.x,
            y: self.session.scroll.y,
            width: self.viewport_width,
            height: self.viewport_height,
        }
    }
}
