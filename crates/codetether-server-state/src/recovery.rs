//! Control-plane URL used by the session recovery API.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_session_recovery::http::HttpState for AppState<R> {
    fn server_url(&self) -> Option<&str> {
        self.inner.config.a2a.server_url.as_deref()
    }
}
