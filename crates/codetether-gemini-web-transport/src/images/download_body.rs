use super::Step;
use crate::images::{format, http, policy};
use anyhow::{Result, anyhow, ensure};
use reqwest::{Response, header::CONTENT_TYPE};
pub(super) async fn read(response: Response) -> Result<Step> {
    let mime = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let body = http::body(response, crate::images::LIMIT).await?;
    if matches!(mime.as_str(), "image/jpeg" | "image/png") {
        return Ok(Step::Image(format::image(body, &mime)?));
    }
    ensure!(
        mime == "text/plain" && body.len() <= 16384,
        "image_response_unexpected"
    );
    let next = std::str::from_utf8(&body)
        .map_err(|_| anyhow!("image_url_invalid"))?
        .trim();
    Ok(Step::Next(policy::image_url(next)?))
}
