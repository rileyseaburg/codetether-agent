//! Bound the per-tool cache used for subsequent image edits.
use super::tool_impl::ImageGenerationTool;
use anyhow::{Result, anyhow};
impl ImageGenerationTool {
    pub(super) fn remember(&self, data_url: &str) -> Result<()> {
        let mut recent = self
            .recent
            .lock()
            .map_err(|_| anyhow!("image cache lock poisoned"))?;
        recent.push_back(data_url.to_owned());
        while recent.len() > super::MAX_EDIT_IMAGES {
            recent.pop_front();
        }
        drop(recent);
        Ok(())
    }
}
