//! Return native Gemini artifacts using the agent's existing image result shape.
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use codetether_gemini_web_transport::images::ImageGeneration;
use codetether_tool_core::ToolResult;
use serde_json::json;

pub(super) fn generated(
    response: &ImageGeneration,
    saved: &super::artifacts::Saved,
    refined: bool,
) -> Result<ToolResult> {
    let image = response
        .images
        .first()
        .ok_or_else(|| anyhow::anyhow!("Gemini returned no images"))?;
    let data_url = format!(
        "data:{};base64,{}",
        image.mime_type,
        STANDARD.encode(&image.bytes)
    );
    let mut result = crate::result::generated_typed(
        data_url,
        saved.images.first().map(|p| p.as_path()),
        &image.mime_type,
    )
    .with_metadata("provider", json!("gemini-web"))
    .with_metadata("generated_image_paths", json!(saved.images))
    .with_metadata("image_width", json!(image.width))
    .with_metadata("image_height", json!(image.height))
    .with_metadata("refined", json!(refined));
    if let Some(path) = &saved.state {
        result = result.with_metadata("refinement_path", json!(path));
        result.output.push_str(&format!(
            "\nFor improvements, call image_gen with provider=gemini-web and refine_from={}",
            path.display()
        ));
    }
    Ok(result)
}
