use super::{args::ImagegenArgs, tool_impl::ImageGenerationTool};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};

impl ImageGenerationTool {
    pub(super) async fn generate(&self, args: ImagegenArgs) -> Result<crate::tool::ToolResult> {
        args.validate()?;
        let images = super::references::resolve(&args, &self.recent).await?;
        let auth = super::credentials::resolve().await?;
        tracing::info!(
            model = super::IMAGE_MODEL,
            edit = !images.is_empty(),
            "Generating image"
        );
        let encoded = self
            .client
            .request(&auth, args.prompt.trim(), images)
            .await?;
        let path = super::artifact::save(&encoded).await?;
        let data_url = format!("data:image/png;base64,{encoded}");
        let mut recent = self
            .recent
            .lock()
            .map_err(|_| anyhow!("image cache lock poisoned"))?;
        recent.push_back(data_url.clone());
        while recent.len() > super::MAX_EDIT_IMAGES {
            recent.pop_front();
        }
        drop(recent);
        let size = STANDARD
            .decode(encoded)
            .map(|bytes| bytes.len())
            .unwrap_or_default();
        tracing::info!(path = %path.display(), size_bytes = size, "Generated image saved");
        Ok(super::result::generated(data_url, &path))
    }
}
