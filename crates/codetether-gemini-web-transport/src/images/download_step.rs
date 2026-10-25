use super::Step;
use crate::images::{ImageClient, client::State, http, policy};
use anyhow::{Result, anyhow, ensure};
use reqwest::{Url, header::LOCATION};

impl ImageClient {
    pub(in crate::images) async fn image_step(&self, url: Url, state: &State) -> Result<Step> {
        let url = policy::image_url(url.as_str())?;
        let mut request = self
            .http
            .get(url.clone())
            .header("Accept", "*/*")
            .header("Origin", crate::images::ORIGIN)
            .header("Referer", "https://gemini.google.com/");
        if url.host_str() == Some("work.fife.usercontent.google.com") {
            request = http::cookie(request, state.base.header(&url, &state.updates))?;
        }
        let response = http::send(request).await?;
        let status = response.status().as_u16();
        if matches!(status, 301 | 302 | 303 | 307 | 308) {
            let location = response
                .headers()
                .get(LOCATION)
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| anyhow!("image_redirect_missing"))?;
            let next = url
                .join(location)
                .map_err(|_| anyhow!("image_url_invalid"))?;
            return Ok(Step::Next(policy::image_url(next.as_str())?));
        }
        ensure!(status == 200, "image_download_http_error");
        super::body::read(response).await
    }
}
