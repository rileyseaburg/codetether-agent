//! Read a successful Gemini response body with the original error context.
use crate::*;
pub(crate) async fn read_response(response: reqwest::Response) -> Result<String> {
    response
        .text()
        .await
        .context("Failed to read Gemini response body")
}
