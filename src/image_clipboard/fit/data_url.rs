//! Data-URL wrapper around [`super::fit`].

use base64::{Engine as _, engine::general_purpose::STANDARD};

/// Fit a `data:image/...;base64,` URL; returns `None` when unchanged.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::image_clipboard::fit::fit_data_url;
///
/// assert!(fit_data_url("data:image/png;base64,bm90IGFuIGltYWdl").is_none());
/// assert!(fit_data_url("https://example.com/a.png").is_none());
/// ```
pub fn fit_data_url(url: &str) -> Option<String> {
    let (header, data) = url.strip_prefix("data:")?.split_once(',')?;
    header.strip_suffix(";base64")?.strip_prefix("image/")?;
    let bytes = STANDARD.decode(data).ok()?;
    let fitted = super::fit(&bytes)?;
    Some(format!(
        "data:{};base64,{}",
        fitted.mime,
        STANDARD.encode(&fitted.bytes)
    ))
}
