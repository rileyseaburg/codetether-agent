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
    if !args.recent_images.is_empty() {
        return tail(&args.recent_images, count, "conversation");
    }
    let mut cache = recent
        .lock()
        .map_err(|_| anyhow::anyhow!("image cache lock poisoned"))?;
    tail(cache.make_contiguous(), count, "generated")
}

fn tail(images: &[String], count: usize, source: &str) -> Result<Vec<String>> {
    if images.len() < count {
        bail!("requested the last {count} {source} images, but only {} were available", images.len());
    }
    Ok(images[images.len() - count..].to_vec())
}