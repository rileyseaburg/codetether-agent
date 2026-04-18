use super::BrowserSession;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

static BROWSER_SERVICE: Lazy<BrowserService> = Lazy::new(BrowserService::default);

pub fn browser_service() -> &'static BrowserService {
    &BROWSER_SERVICE
}

pub struct BrowserService {
    session: Mutex<Option<BrowserSession>>,
}

impl Default for BrowserService {
    fn default() -> Self {
        Self {
            session: Mutex::new(None),
        }
    }
}

impl BrowserService {
    pub fn clear(&self) {
        self.session.lock().take();
    }

    pub fn session(&self) -> BrowserSession {
        if let Some(session) = self.session.lock().clone() {
            return session;
        }
        let session = BrowserSession::new();
        *self.session.lock() = Some(session.clone());
        session
    }
}
