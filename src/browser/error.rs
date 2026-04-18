use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("browser command is not implemented")]
    NotImplemented,
    #[error("navigation timed out")]
    NavigationTimeout,
    #[error("element not found: {0}")]
    ElementNotFound(String),
    #[error("javascript exception: {0}")]
    JsException(String),
    #[error("browser session not started")]
    SessionNotStarted,
    #[error("tab is closed")]
    TabClosed,
    #[error("browser crashed or connection was lost")]
    BrowserCrashed,
    #[error("browser operation failed: {0}")]
    OperationFailed(String),
}

impl From<anyhow::Error> for BrowserError {
    fn from(error: anyhow::Error) -> Self {
        Self::OperationFailed(error.to_string())
    }
}
