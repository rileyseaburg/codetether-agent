//! Validate remote image references without network access or URL rewriting.

use crate::tool::agent::collaboration_runtime::message_input::MessageImage;
use anyhow::{Context, Result, ensure};

#[cfg(test)]
#[path = "../tests/image_reference.rs"]
mod tests;

pub(super) fn remote(value: &str) -> Result<MessageImage> {
    let url = reqwest::Url::parse(value).context("image_url must be a valid image URL")?;
    ensure!(
        matches!(url.scheme(), "http" | "https"),
        "image_url scheme must be http, https, or a base64 image data URL"
    );
    ensure!(
        url.host_str().is_some()
            && value.split_once("://").is_some_and(|(scheme, rest)| {
                scheme.eq_ignore_ascii_case(url.scheme()) && !rest.starts_with(['/', '?', '#'])
            })
            && !value.chars().any(|c| c.is_whitespace() || c.is_control())
            && !value.contains('\\'),
        "image_url must be an absolute HTTP(S) URL without whitespace or backslashes"
    );
    Ok(MessageImage {
        data_url: value.to_owned(),
        mime_type: None,
    })
}
