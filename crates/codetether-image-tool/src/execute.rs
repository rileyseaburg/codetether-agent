//! execute for image loading.
use super::*;
impl ImageTool {
    pub(super) async fn load(&self, args: Value) -> Result<ToolResult> {
        let input: ImageToolInput = serde_json::from_value(args)?;

        let (data_url, mime_type, size_bytes, source) =
            if input.path.starts_with("http://") || input.path.starts_with("https://") {
                let response = reqwest::get(&input.path).await?;
                if !response.status().is_success() {
                    return Ok(ToolResult::error(format!(
                        "Failed to fetch image from URL: HTTP {}",
                        response.status()
                    )));
                }
                let mt = response
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| Self::detect_mime_type(&input.path).to_string());
                let bytes = response.bytes().await?;
                let url = Self::encode_as_data_url(&bytes, &mt);
                (url, mt, bytes.len(), input.path.clone())
            } else {
                let path = std::path::Path::new(&input.path);
                if !path.exists() {
                    return Ok(ToolResult::error(format!(
                        "Image file not found: {}",
                        input.path
                    )));
                }
                let data = tokio::fs::read(path).await?;
                let mt = Self::detect_mime_type(&input.path).to_string();
                let url = Self::encode_as_data_url(&data, &mt);
                (url, mt, data.len(), input.path.clone())
            };

        let detail = input.detail.unwrap_or_else(|| "auto".to_string());
        let summary =
            format!("Image loaded: {source} ({size_bytes} bytes, {mime_type}, detail={detail})");
        Ok(ToolResult::success(summary).with_metadata(
            "image_data_url",
            serde_json::json!({
                "data_url": data_url,
                "mime_type": mime_type,
                "detail": detail,
            }),
        ))
    }
}
