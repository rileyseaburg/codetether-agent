//! execute for image generation and editing.
use super::{args::ImagegenArgs, tool_impl::ImageGenerationTool};
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};

impl ImageGenerationTool {
    pub(super) async fn generate(
        &self,
        args: ImagegenArgs,
    ) -> Result<codetether_tool_core::ToolResult> {
        args.validate()?;
        let images = super::references::resolve(&args, &self.recent).await?;
        let auth = super::credentials::resolve().await?;
        tracing::info!(target: "codetether_agent::tool::image_generation::execute",
            model = super::IMAGE_MODEL,
            edit = !images.is_empty(),
            "Generating image"
        );
        let encoded = self
            .client
            .request(&auth, args.prompt.trim(), images)
            .await?;
        let path = super::artifact::save(
            &encoded,
            args.session_id.as_deref(),
            args.call_id.as_deref(),
        )
        .await
        .map_err(|error| {
            tracing::warn!(target: "codetether_agent::tool::image_generation::execute", error = %error, "Failed to save generated image");
            error
        })
        .ok();
        let data_url = format!("data:image/png;base64,{encoded}");
        self.remember(&data_url)?;
        let size = STANDARD
            .decode(encoded)
            .map(|bytes| bytes.len())
            .unwrap_or_default();
        if let Some(path) = &path {
            tracing::info!(target: "codetether_agent::tool::image_generation::execute", path = %path.display(), size_bytes = size, "Generated image saved");
        }
        Ok(super::result::generated(data_url, path.as_deref()))
    }
}
