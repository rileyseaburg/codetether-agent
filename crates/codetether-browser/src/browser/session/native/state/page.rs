use tetherscript::browser_agent::BrowserPage;
use tetherscript::browser_session::BrowserSession as TetherSession;

pub(super) struct NativePage {
    pub session: TetherSession,
    pub viewport_width: i64,
    pub viewport_height: i64,
}

impl NativePage {
    pub fn new() -> Self {
        Self::from_page(BrowserPage::new(TetherSession::new()))
    }

    pub fn from_page(page: BrowserPage) -> Self {
        Self {
            session: page.session,
            viewport_width: page.viewport_width,
            viewport_height: page.viewport_height,
        }
    }

    pub fn page(&self) -> BrowserPage {
        let mut page = BrowserPage::new(self.session.clone());
        page.viewport_width = self.viewport_width;
        page.viewport_height = self.viewport_height;
        page
    }

    pub fn replace(&mut self, page: BrowserPage) {
        *self = Self::from_page(page);
    }
}
