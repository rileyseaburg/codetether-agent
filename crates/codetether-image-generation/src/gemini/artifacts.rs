//! Save every generated image and its private conversation state as a new version.
use anyhow::{Result, bail};
use codetether_gemini_web_transport::images::ImageGeneration;
use std::path::PathBuf;

pub(super) struct Saved {
    pub(super) images: Vec<PathBuf>,
    pub(super) state: Option<PathBuf>,
}

pub(super) fn save(generation: &ImageGeneration, root: PathBuf) -> Result<Saved> {
    if generation.images.is_empty() {
        bail!("Gemini returned no image bytes");
    }
    super::private_files::directory(&root)?;
    let mut images = Vec::new();
    for (index, image) in generation.images.iter().enumerate() {
        if !matches!(image.extension.as_str(), "jpg" | "png") {
            bail!("Unsupported Gemini image format");
        }
        let path = root.join(format!("image-{}.{}", index + 1, image.extension));
        super::private_files::write(&path, &image.bytes)?;
        images.push(path);
    }
    let state = generation
        .continuation
        .as_ref()
        .map(|value| super::state::save(&root, value))
        .transpose()?;
    Ok(Saved { images, state })
}
