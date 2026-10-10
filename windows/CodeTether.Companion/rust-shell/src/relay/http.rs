//! Cancellable, bounded device HTTP; no redirect or automatic replay.
use super::{Device, Error};
use reqwest::{Client, RequestBuilder, header::HeaderValue};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use zeroize::Zeroizing;

pub(super) fn client() -> Result<Client, Error> {
    Client::builder()
        .https_only(true)
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|_| Error::Unavailable)
}
impl Device {
    pub(super) fn live(&self, cancel: &CancellationToken) -> Result<(), Error> {
        if cancel.is_cancelled() {
            return Err(Error::Cancelled);
        }
        if self.expired() {
            return Err(Error::Revoked);
        }
        Ok(())
    }
    pub(super) fn authorize(&self, request: RequestBuilder) -> Result<RequestBuilder, Error> {
        let value = Zeroizing::new(format!("Bearer {}", self.token.as_str()));
        let mut header = HeaderValue::from_str(&value).map_err(|_| Error::InvalidInput)?;
        header.set_sensitive(true);
        Ok(request
            .header("Authorization", header)
            .header("Cache-Control", "no-store"))
    }
}
pub(super) async fn receive<T: DeserializeOwned>(
    request: RequestBuilder,
    expected: u16,
    cancel: &CancellationToken,
) -> Result<T, Error> {
    tokio::select! { biased;
        _ = cancel.cancelled() => Err(Error::Cancelled),
        result = async {
            let mut response = request.send().await.map_err(|_| Error::Unavailable)?;
            let status = response.status().as_u16();
            if status != expected { return Err(Error::from_status(status)); }
            let mut bytes = Zeroizing::new(Vec::with_capacity(8192));
            while let Some(chunk) = response.chunk().await.map_err(|_| Error::Unavailable)? {
                if chunk.len() > 8192 - bytes.len() { return Err(Error::InvalidResponse); }
                bytes.extend_from_slice(&chunk);
            }
            serde_json::from_slice(&bytes).map_err(|_| Error::InvalidResponse)
        } => result,
    }
}
