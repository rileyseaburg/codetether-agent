//! Validate the bounded rotation response before persisting renewed cookies.
use super::{
    cookies::now,
    http,
    rotation::{self, Rotation},
};
use anyhow::{Result, bail, ensure};
use reqwest::{Response, header::SET_COOKIE};

pub(super) async fn parse(response: Response) -> Result<Vec<Rotation>> {
    if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
        bail!(super::renewal_throttle::Throttled);
    }
    ensure!(
        response.status().as_u16() == 200,
        "cookie_refresh_rejected: HTTP {}",
        response.status().as_u16()
    );
    let updates: Vec<_> = response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|h| h.to_str().ok())
        .filter_map(rotation::from_header)
        .collect();
    let _ = http::body(response, 65536).await?;
    ensure!(
        updates
            .iter()
            .any(|v| v.name == "__Secure-1PSIDTS" && v.expires > now() && !v.value.is_empty()),
        "cookie_refresh_no_updates"
    );
    Ok(updates)
}
