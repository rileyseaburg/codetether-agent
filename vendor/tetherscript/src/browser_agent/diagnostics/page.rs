//! BrowserPage diagnostics API.

use crate::browser_agent::page::BrowserPage;

use super::types::BrowserDebugReport;

impl BrowserPage {
    /// Build a production-debug report from native browser runtime signals.
    ///
    /// # Returns
    ///
    /// Console errors, page errors, failed requests, source-map status, and
    /// React/framework markers collected from the deterministic page state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://app", "<div id='root'></div>");
    /// let report = page.production_debug_report();
    /// assert!(report.parity.native_engine);
    /// ```
    pub fn production_debug_report(&self) -> BrowserDebugReport {
        super::report::build(self)
    }
}
