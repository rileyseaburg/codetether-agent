//! Choose one native clipboard representation without reading it twice.
use codetether_image::ImageAttachment;

pub(super) enum Content {
    Text(String),
    Image(ImageAttachment),
}

pub(super) fn read(
    prefer_image: bool,
    text: impl FnOnce() -> Option<String>,
    image: impl FnOnce() -> Option<ImageAttachment>,
) -> Option<Content> {
    if prefer_image {
        image()
            .map(Content::Image)
            .or_else(|| text().map(Content::Text))
    } else {
        text()
            .map(Content::Text)
            .or_else(|| image().map(Content::Image))
    }
}
