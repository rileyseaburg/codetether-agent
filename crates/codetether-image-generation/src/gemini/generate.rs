//! Run the native Gemini transport and return normal image tool metadata.
use crate::args::ImagegenArgs;
use anyhow::{Result, bail};
use codetether_tool_core::ToolResult;

pub(crate) async fn generate(args: &ImagegenArgs) -> Result<ToolResult> {
    let previous = args
        .refine_from
        .as_deref()
        .map(super::state::load)
        .transpose()?;
    let data = codetether_config::Config::data_dir()
        .ok_or_else(|| anyhow::anyhow!("CodeTether data directory unavailable"))?;
    let cookies = super::credentials::load().await?;
    let client = super::client::create(&cookies, &data)?;
    let response = client
        .generate(args.prompt.trim(), previous.as_ref())
        .await?;
    if let Some(previous) = &previous
        && response.continuation.as_ref().map(|c| &c.conversation_id)
            != Some(&previous.conversation_id)
    {
        bail!("Gemini refinement did not preserve the original conversation");
    }
    let root = data
        .join("generated_images")
        .join("gemini-web")
        .join(uuid::Uuid::new_v4().to_string());
    let saved = super::artifacts::save(&response, root)?;
    tracing::info!(
        provider = "gemini-web",
        image_count = response.images.len(),
        refined = previous.is_some(),
        "Generated images"
    );
    super::result::generated(&response, &saved, previous.is_some())
}
