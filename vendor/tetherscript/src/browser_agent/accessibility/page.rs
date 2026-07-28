//! Browser page accessibility snapshot API.

use crate::browser_agent::page::BrowserPage;

use super::{build, AccessibilitySnapshot};

impl BrowserPage {
    /// Return a deterministic accessibility snapshot for the current DOM.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://a11y", "<button>Save</button>");
    /// assert_eq!(page.accessibility_snapshot().focus_order, vec!["path:0"]);
    /// ```
    pub fn accessibility_snapshot(&self) -> AccessibilitySnapshot {
        build::snapshot(&self.session.document)
    }
}
