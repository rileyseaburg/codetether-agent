use super::{BrowserSession, SessionMode, SessionRuntime};
use crate::browser::BrowserError;
use chromiumoxide::cdp::browser_protocol::target::TargetId;
use chromiumoxide::{browser::Browser, page::Page};
use std::sync::Arc;
use tokio::sync::Mutex;

pub(super) struct RuntimeAccess {
    pub browser: Arc<Mutex<Browser>>,
    pub current_page: Arc<Mutex<Option<Page>>>,
    pub mode: SessionMode,
    pub tab_order: Arc<Mutex<Vec<TargetId>>>,
}

pub(super) async fn current_page(session: &BrowserSession) -> Result<Page, BrowserError> {
    runtime(session)
        .await?
        .current_page
        .lock()
        .await
        .clone()
        .ok_or(BrowserError::TabClosed)
}

pub(super) async fn runtime(session: &BrowserSession) -> Result<RuntimeAccess, BrowserError> {
    let slot = session.inner.runtime.lock().await;
    let runtime = slot.as_ref().ok_or(BrowserError::SessionNotStarted)?;
    if !runtime.is_alive() {
        return Err(BrowserError::BrowserCrashed);
    }
    Ok(RuntimeAccess::from(runtime))
}

impl From<&SessionRuntime> for RuntimeAccess {
    fn from(runtime: &SessionRuntime) -> Self {
        Self {
            browser: Arc::clone(&runtime.browser),
            current_page: Arc::clone(&runtime.current_page),
            mode: runtime.mode,
            tab_order: Arc::clone(&runtime.tab_order),
        }
    }
}
