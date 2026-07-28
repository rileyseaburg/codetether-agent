//! Browser page canvas inspection APIs.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::query::locate;

use super::canvas_model::{CanvasCommand, CanvasSurface};
use super::canvas_parse::surface_from_element;

impl BrowserPage {
    /// Return the deterministic surface snapshot for one canvas.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the locator does not resolve to one canvas.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::{BrowserPage, Locator};
    ///
    /// let page = BrowserPage::from_html("mem://canvas", "<canvas id='c'></canvas>");
    /// let surface = page.canvas_surface(&Locator::css("#c")).unwrap();
    /// assert_eq!((surface.width, surface.height), (300, 150));
    /// ```
    pub fn canvas_surface(&self, locator: &Locator) -> Result<CanvasSurface, String> {
        let matches = locate(&self.session.document, locator);
        if matches.is_empty() {
            return Err(format!("locator {:?} matched no elements", locator.kind));
        }
        if locator.strict && matches.len() != 1 {
            return Err(format!(
                "locator {:?} matched {} elements",
                locator.kind,
                matches.len()
            ));
        }
        let element = &matches[0].element;
        if element.tag != "canvas" {
            return Err(format!(
                "locator {:?} resolved to <{}>, expected <canvas>",
                locator.kind, element.tag
            ));
        }
        Ok(surface_from_element(element))
    }

    /// Return the deterministic command log for one canvas.
    ///
    /// # Errors
    ///
    /// Returns `Err` when [`BrowserPage::canvas_surface`] fails.
    pub fn canvas_commands(&self, locator: &Locator) -> Result<Vec<CanvasCommand>, String> {
        Ok(self.canvas_surface(locator)?.commands)
    }
}
