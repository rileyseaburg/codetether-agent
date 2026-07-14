use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ImagesResponse {
    data: Vec<ImageData>,
}

#[derive(Deserialize)]
struct ImageData {
    b64_json: String,
}

#[derive(Deserialize)]
pub(super) struct ApiErrorEnvelope {
    error: ApiError,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
    code: Option<String>,
}

impl ImagesResponse {
    pub(super) fn into_base64(self) -> Result<String> {
        self.data
            .into_iter()
            .next()
            .map(|item| item.b64_json)
            .ok_or_else(|| anyhow!("image generation returned no image data"))
    }
}

impl ApiErrorEnvelope {
    pub(super) fn message(self) -> String {
        self.error.code.map_or(self.error.message.clone(), |code| {
            format!("{} ({code})", self.error.message)
        })
    }
}
