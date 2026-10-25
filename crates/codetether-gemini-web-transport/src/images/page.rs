use super::{
    ImageClient,
    client::State,
    http,
    tokens::{self, Tokens},
};
use anyhow::{Result, anyhow, bail, ensure};
use reqwest::{Url, header::LOCATION};

impl ImageClient {
    pub(super) async fn page(&self, state: &State) -> Result<Tokens> {
        let mut url = Url::parse(super::ORIGIN).map_err(|_| anyhow!("image_url_invalid"))?;
        for _ in 0..4 {
            let request = http::cookie(
                self.http.get(url.clone()),
                state.base.header(&url, &state.updates),
            )?;
            let response = http::send(request)
                .await
                .map_err(|error| anyhow!("image_bootstrap: {error}"))?;
            if response.status().is_redirection() {
                let location = response
                    .headers()
                    .get(LOCATION)
                    .and_then(|v| v.to_str().ok())
                    .ok_or_else(|| anyhow!("image_session_redirect_invalid"))?;
                let next = url
                    .join(location)
                    .map_err(|_| anyhow!("image_session_redirect_invalid"))?;
                ensure!(
                    next.origin() == url.origin()
                        && next.username().is_empty()
                        && next.password().is_none(),
                    "image_session_redirect_disallowed"
                );
                url = next;
                continue;
            }
            ensure!(
                response.status().as_u16() == 200,
                "image_session_http_error"
            );
            let body = http::body(response, super::LIMIT).await?;
            let html = std::str::from_utf8(&body)
                .map_err(|_| anyhow!("image_session_encoding_invalid"))?;
            return tokens::parse(html, url.path());
        }
        bail!("image_session_redirect_limit")
    }
}
