//! Native entry point for shared human-input ingestion.
mod host;
use super::Runner;
use crate::session::ImageAttachment;
impl Runner<'_> {
    pub(crate) async fn accept(
        &mut self,
        message: &str,
        images: Vec<ImageAttachment>,
    ) -> anyhow::Result<()> {
        codetether_prompt_loop::input::accept(self, message, images).await
    }
}
