//! Persisted page state for the native backend.
//!
//! `BrowserPage` itself is not Send because it owns JavaScript runtime handles,
//! so this type stores only the session snapshot and viewport dimensions.

use tetherscript::browser_agent::BrowserPage;
use tetherscript::browser_session::BrowserSession as TetherSession;

/// Send-friendly representation of one native browser tab.
pub(in crate::browser::session) struct NativePage {
    /// Persistent TetherScript browser session state.
    pub session: TetherSession,
    /// Current viewport width in CSS pixels.
    pub viewport_width: i64,
    /// Current viewport height in CSS pixels.
    pub viewport_height: i64,
}

impl NativePage {
    /// Create a blank page.
    pub fn new() -> Self {
        Self::from_page(BrowserPage::new(TetherSession::new()))
    }

    /// Create page state from HTML and run page scripts synchronously.
    pub fn from_html(url: String, html: String) -> Self {
        let mut page = BrowserPage::from_html(url, html);
        let _ = page.run_scripts();
        Self::from_page(page)
    }

    /// Convert a transient TetherScript page into stored state.
    pub fn from_page(page: BrowserPage) -> Self {
        Self {
            session: page.session,
            viewport_width: page.viewport_width,
            viewport_height: page.viewport_height,
        }
    }

    /// Build a transient TetherScript page from stored state.
    pub fn page(&self) -> BrowserPage {
        let mut page = BrowserPage::new(self.session.clone());
        page.viewport_width = self.viewport_width;
        page.viewport_height = self.viewport_height;
        page
    }

    /// Replace stored state from a transient TetherScript page.
    pub fn replace(&mut self, page: BrowserPage) {
        *self = Self::from_page(page);
    }
}
