//! Build optional metadata and privacy fields around the video part.
use super::params::UploadVideoParams;
pub(super) fn build(
    part: reqwest::multipart::Part,
    params: &UploadVideoParams,
) -> reqwest::multipart::Form {
    let privacy = params.privacy_status.as_deref().unwrap_or("unlisted");

    let mut form = reqwest::multipart::Form::new()
        .part("video", part)
        .text("title", params.title.clone())
        .text("privacy_status", privacy.to_string());

    if let Some(ref desc) = params.description {
        form = form.text("description", desc.clone());
    }
    if let Some(ref tags) = params.tags {
        form = form.text("tags", tags.clone());
    }
    if let Some(ref category) = params.category_id {
        form = form.text("category_id", category.clone());
    }

    form
}
