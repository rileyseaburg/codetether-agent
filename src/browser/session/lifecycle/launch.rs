use super::{browser, handler, mode::StartMode};
use crate::browser::{BrowserError, session::SessionRuntime};
use std::sync::Arc;
use tokio::sync::Mutex;

pub(super) async fn runtime(mode: StartMode) -> Result<SessionRuntime, BrowserError> {
    let (browser, handler, mode) = browser::connect_or_launch(mode).await?;
    let (alive, shutdown, handler_task) = handler::spawn(handler);
    let page = match browser.new_page("about:blank").await {
        Ok(page) => page,
        Err(error) => {
            let _ = shutdown.send(true);
            handler_task.abort();
            let _ = handler_task.await;
            return Err(error.into());
        }
    };
    Ok(SessionRuntime {
        alive,
        browser: Arc::new(Mutex::new(browser)),
        current_page: Arc::new(Mutex::new(page)),
        handler_task,
        mode,
        shutdown,
    })
}
