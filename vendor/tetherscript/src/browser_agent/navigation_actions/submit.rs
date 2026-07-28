//! Public requestSubmit-style page action.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::keyboard_escape::node;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve;
use crate::js::JsValue;

impl BrowserPage {
    /// Submit a located `<form>` through its persistent runtime.
    ///
    /// # Errors
    ///
    /// Returns an error if the locator does not resolve to exactly one visible
    /// form element or if the page runtime rejects the submit script.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::{BrowserPage, Locator};
    ///
    /// let mut page = BrowserPage::from_html(
    ///     "https://example.test/app",
    ///     "<form id='f' action='/next'><input name='q' value='rust'></form>",
    /// );
    /// page.request_submit(&Locator::css("#f")).unwrap();
    ///
    /// assert_eq!(page.session.url, "https://example.test/next?q=rust");
    /// ```
    pub fn request_submit(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let resolved = resolve::resolve(&self.session, self.viewport_width, locator)?;
        if !resolved.dom.element.tag.eq_ignore_ascii_case("form") {
            return Err(format!(
                "locator {:?} did not resolve to a form",
                locator.kind
            ));
        }
        let result = self.eval_js(&format!(
            "let n={}; n.requestSubmit()",
            node(&resolved.dom.path)
        ))?;
        if result != JsValue::Bool(false) {
            if let Some(url) = super::form::form_target(
                &self.session.document,
                &resolved.dom.path,
                &self.session.url,
            ) {
                super::commit::document(self, url, "request_submit");
            }
        }
        Ok(ActionReport::new(
            "request_submit",
            format!("{:?}", locator.kind),
            resolved.bounds,
            ActionabilityReport::new(false),
        ))
    }
}
