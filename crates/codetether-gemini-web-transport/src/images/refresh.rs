use super::rotation_fresh::usable;
use super::{ImageClient, client::State, cookies::now, http, rotation};
use anyhow::{Result, anyhow, ensure};
use reqwest::Url;

impl ImageClient {
    pub(super) async fn refresh(&self, state: &mut State, force: bool) -> Result<bool> {
        if let Some(cache) = self.cache.load().filter(|_| state.rotated_at == 0) {
            state.updates = cache.cookies;
            state.rotated_at = cache.updated;
        }
        if !force && usable(state.rotated_at, &state.updates, now()) {
            return Ok(true);
        }
        let url = Url::parse(super::ROTATE).map_err(|_| anyhow!("image_url_invalid"))?;
        let value = state.base.header(&url, &state.updates);
        ensure!(
            value.split("; ").any(|v| v
                .strip_prefix("__Secure-1PSID=")
                .is_some_and(|v| !v.is_empty())),
            "cookie_refresh_missing_session"
        );
        let request = self
            .http
            .post(url)
            .header("Content-Type", "application/json")
            .header("Origin", "https://accounts.google.com")
            .body("[000,\"-0000000000000000000\"]");
        let response = http::send(http::cookie(request, value)?).await?;
        let updates = super::rotation_response::parse(response).await?;
        rotation::apply(&mut state.updates, updates);
        state.rotated_at = now();
        self.cache.save(state.rotated_at, &state.updates)?;
        Ok(false)
    }
}
