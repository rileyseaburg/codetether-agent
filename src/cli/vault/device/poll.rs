//! Bounded RFC 8628 polling; provider tokens are never printed or persisted.

use anyhow::{Result, bail};
use std::time::{Duration, Instant};

pub(super) async fn wait(
    metadata: &super::discovery::Metadata,
    grant: &super::start::Grant,
    client_id: &str,
) -> Result<String> {
    let deadline = Instant::now() + Duration::from_secs(grant.expires_in.min(1800));
    let client = crate::secrets::login::http::client()?;
    let mut interval = grant.interval.clamp(1, 60);
    loop {
        if Instant::now() >= deadline {
            bail!("Device login expired; start a new login");
        }
        tokio::time::sleep(
            Duration::from_secs(interval).min(deadline.saturating_duration_since(Instant::now())),
        )
        .await;
        if Instant::now() >= deadline {
            bail!("Device login expired; start a new login");
        }
        match super::token_request::request(&client, metadata, grant, client_id).await? {
            super::token_reply::Outcome::Pending => {}
            super::token_reply::Outcome::SlowDown => interval = (interval + 5).min(120),
            super::token_reply::Outcome::Token(token) => return Ok(token),
        }
    }
}
