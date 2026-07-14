use super::args::ImagegenArgs;
use anyhow::{Result, bail};
use std::{collections::VecDeque, sync::Mutex};

pub(super) async fn resolve(
    args: &ImagegenArgs,
    recent: &Mutex<VecDeque<String>>,
) -> Result<Vec<String>> {
    if let Some(paths) = args
        .referenced_image_paths
        .as_deref()
        .filter(|paths| !paths.is_empty())
    {
        let mut images = Vec::with_capacity(paths.len());
        for path in paths {
            images.push(super::reference_file::load(path).await?);
        }
        return Ok(images);
    }
    let Some(count) = args.num_last_images_to_include else {
        return Ok(Vec::new());
    };
    let cache = recent
        .lock()
        .map_err(|_| anyhow::anyhow!("image cache lock poisoned"))?;
    if cache.len() < count {
        bail!(
            "requested the last {count} generated images, but only {} were available",
            cache.len()
        );
    }
    Ok(cache.iter().rev().take(count).rev().cloned().collect())
}
