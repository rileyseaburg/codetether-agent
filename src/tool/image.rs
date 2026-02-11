//! Image tool for loading and encoding images
//!
//! This tool allows agents to load images from files or URLs and encode them
//! as base64 data URLs for use with vision-capable models.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Input parameters for the image tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageToolInput {
    /// Path to the image file or URL
    pub path: String,
    /// Optional detail level (low, high, auto) - for providers that support it
    #[serde(default)]
    pub detail: Option<String>,
}

/// Tool for loading images from files or URLs
pub struct ImageTool;

impl ImageTool {
    pub fn new() -> Self {
        Self
    }

    /// Detect MIME type from file extension
    fn detect_mime_type(path: &str) -> &'static str {
        let lower = path.to_lowercase();
        if lower.ends_with(".png") {
            "image/png"
        } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
            "image/jpeg"
        } else if lower.ends_with(".gif") {
            "image/gif"
        } else if lower.ends_with(".webp") {
            "image/webp"
        } else if lower.ends_with(".svg") {
            "image/svg+xml"
        } else if lower.ends_with(".bmp") {
            "image/bmp"
        } else if lower.ends_with(".tiff") || lower.ends_with(".tif") {
            "image/tiff"
        } else {
            "image/jpeg" // Default fallback
        }
    }

    /// Encode image data as base64 data URL
    fn encode_as_data_url(data: &[u8], mime_type: &str) -> String {
        let base64 = STANDARD.encode(data);
        format!("data:{};base64,{}", mime_type, base64)
    }
}

#[async_trait]
impl Tool for ImageTool {
    fn id(&self) -> &str {
        "image"
    }

    fn name(&self) -> &str {
        "Image Loader"
    }

    fn description(&self) -> &str {
        "Load an image from a file path or URL and encode it for use with vision-capable models. \
         Supports PNG, JPEG, GIF, WebP, SVG, BMP, and TIFF formats. \
         Returns a base64-encoded data URL that can be used in image content parts."
    }

    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the image file (absolute or relative) or URL"
                },
                "detail": {
                    "type": "string",
                    "enum": ["low", "high", "auto"],
                    "description": "Optional detail level for vision models (low=512x512, high=full resolution with 512x512 tiles)"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let input: ImageToolInput = serde_json::from_value(args)?;

        // Check if path is a URL
        let data = if input.path.starts_with("http://") || input.path.starts_with("https://") {
            // Fetch image from URL
            let response = reqwest::get(&input.path).await?;

            if !response.status().is_success() {
                return Ok(ToolResult::error(format!(
                    "Failed to fetch image from URL: HTTP {}",
                    response.status()
                )));
            }

            // Try to get MIME type from response headers
            let mime_type = response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
                .unwrap_or_else(|| Self::detect_mime_type(&input.path).to_string());

            let bytes = response.bytes().await?;
            let data_url = Self::encode_as_data_url(&bytes, &mime_type);

            serde_json::json!({
                "data_url": data_url,
                "mime_type": mime_type,
                "size_bytes": bytes.len(),
                "source": input.path,
                "detail": input.detail.unwrap_or_else(|| "auto".to_string())
            })
        } else {
            // Load from file path
            let path = std::path::Path::new(&input.path);

            if !path.exists() {
                return Ok(ToolResult::error(format!(
                    "Image file not found: {}",
                    input.path
                )));
            }

            let data = tokio::fs::read(path).await?;
            let mime_type = Self::detect_mime_type(&input.path);
            let data_url = Self::encode_as_data_url(&data, mime_type);

            serde_json::json!({
                "data_url": data_url,
                "mime_type": mime_type,
                "size_bytes": data.len(),
                "source": input.path,
                "detail": input.detail.unwrap_or_else(|| "auto".to_string())
            })
        };

        Ok(ToolResult::success(serde_json::to_string_pretty(&data)?))
    }
}

impl Default for ImageTool {
    fn default() -> Self {
        Self::new()
    }
}
