use crate::browser::BrowserError;
use std::path::Path;

pub(super) async fn html(url: &str) -> Result<String, BrowserError> {
    if matches!(url, "" | "about:blank") {
        return Ok(String::new());
    }
    if let Some(rest) = url.strip_prefix("data:text/html,") {
        return Ok(rest.replace("%20", " "));
    }
    if let Some(path) = url.strip_prefix("file://") {
        return read_file(path).await;
    }
    if Path::new(url).exists() {
        return read_file(url).await;
    }
    http_get(url).await
}

pub(super) async fn http_get(url: &str) -> Result<String, BrowserError> {
    let response = reqwest::get(url).await.map_err(map)?;
    response.text().await.map_err(map)
}

pub(super) fn map(error: reqwest::Error) -> BrowserError {
    BrowserError::OperationFailed(error.to_string())
}

async fn read_file(path: &str) -> Result<String, BrowserError> {
    tokio::fs::read_to_string(path)
        .await
        .map_err(|error| BrowserError::OperationFailed(error.to_string()))
}
