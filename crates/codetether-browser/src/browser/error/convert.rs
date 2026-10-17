//! Convert lower-level failures into browser operation errors.
use super::BrowserError;

impl From<anyhow::Error> for BrowserError {
    fn from(error: anyhow::Error) -> Self {
        Self::OperationFailed(error.to_string())
    }
}

impl From<serde_json::Error> for BrowserError {
    fn from(error: serde_json::Error) -> Self {
        Self::OperationFailed(error.to_string())
    }
}
