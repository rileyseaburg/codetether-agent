//! API error decoding with a bounded Unicode-safe body preview.
use serde::Deserialize;

#[derive(Deserialize)]
struct ZaiError {
    error: ErrorDetail,
}
#[derive(Deserialize)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
}

pub(crate) fn message(status: reqwest::StatusCode, text: &str) -> String {
    serde_json::from_str::<ZaiError>(text)
        .map(|error| {
            format!(
                "Z.AI API error: {} ({:?})",
                error.error.message, error.error.error_type
            )
        })
        .unwrap_or_else(|_| {
            format!(
                "Z.AI API error: {status} {}",
                crate::preview::text(text, 200)
            )
        })
}
