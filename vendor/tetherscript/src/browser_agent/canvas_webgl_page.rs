//! Browser page WebGL inspection APIs.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::query::locate;

use super::canvas_webgl_model::WebGlContextSnapshot;
use super::canvas_webgl_parse::snapshot_from_element;

impl BrowserPage {
    /// Return the deterministic WebGL metadata snapshot for one canvas.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the locator is not one canvas with WebGL metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::{BrowserPage, Locator};
    ///
    /// let mut page = BrowserPage::from_html("mem://webgl", "<canvas id='c'></canvas>");
    /// page.eval_js("document.getElementById('c').getContext('webgl')").unwrap();
    /// let snapshot = page.webgl_context(&Locator::css("#c")).unwrap();
    /// assert_eq!(snapshot.version, 1);
    /// ```
    pub fn webgl_context(&self, locator: &Locator) -> Result<WebGlContextSnapshot, String> {
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
                "locator {:?} resolved to <{}>",
                locator.kind, element.tag
            ));
        }
        snapshot_from_element(element).ok_or_else(|| {
            format!(
                "locator {:?} resolved to canvas without WebGL metadata",
                locator.kind
            )
        })
    }
}
