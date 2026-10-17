//! Build and send the create_podcast request.
use super::{PodcastTool, params::CreatePodcastParams, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl PodcastTool {
    pub(super) async fn create_podcast(&self, params: &CreatePodcastParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts");

        let mut form = reqwest::multipart::Form::new()
            .text("title", params.title.clone())
            .text("description", params.description.clone())
            .text(
                "author",
                params
                    .author
                    .clone()
                    .unwrap_or_else(|| "CodeTether Agent".into()),
            );

        if let Some(ref email) = params.email {
            form = form.text("email", email.clone());
        }
        if let Some(ref category) = params.category {
            form = form.text("category", category.clone());
        }
        if let Some(ref subcategory) = params.subcategory {
            form = form.text("subcategory", subcategory.clone());
        }
        if let Some(ref language) = params.language {
            form = form.text("language", language.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Create podcast request failed: {e}"))?;

        super::create_podcast_result::read(resp, params).await
    }
}
