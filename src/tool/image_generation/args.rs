use anyhow::{Result, bail};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub(super) struct ImagegenArgs {
    pub(super) prompt: String,
    pub(super) referenced_image_paths: Option<Vec<PathBuf>>,
    pub(super) num_last_images_to_include: Option<usize>,
}

impl ImagegenArgs {
    pub(super) fn validate(&self) -> Result<()> {
        if self.prompt.trim().is_empty() {
            bail!("`prompt` must not be empty");
        }
        let paths = self.referenced_image_paths.as_deref().unwrap_or_default();
        if paths.len() > super::MAX_EDIT_IMAGES {
            bail!("`referenced_image_paths` must contain at most 5 paths");
        }
        if !paths.is_empty() && self.num_last_images_to_include.is_some() {
            bail!("provide only one image selector");
        }
        if let Some(count) = self.num_last_images_to_include
            && !(1..=super::MAX_EDIT_IMAGES).contains(&count)
        {
            bail!("`num_last_images_to_include` must be between 1 and 5");
        }
        Ok(())
    }
}
