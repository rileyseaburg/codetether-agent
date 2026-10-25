use super::{GeneratedImage, ImageClient, client::State, policy};
use anyhow::{Result, bail, ensure};
use reqwest::Url;
use std::collections::HashSet;

pub(super) enum Step {
    Next(Url),
    Image(GeneratedImage),
}
impl ImageClient {
    pub(super) async fn download(&self, reference: &str, state: &State) -> Result<GeneratedImage> {
        let mut url = policy::rendition(reference)?;
        let mut visited = HashSet::new();
        for _ in 0..6 {
            ensure!(
                visited.insert(url.as_str().to_owned()),
                "image_redirect_loop"
            );
            match self.image_step(url, state).await? {
                Step::Next(next) => url = next,
                Step::Image(image) => return Ok(image),
            }
        }
        bail!("image_hop_limit")
    }
}
#[path = "download_step.rs"]
mod step;

#[path = "download_body.rs"]
mod body;
