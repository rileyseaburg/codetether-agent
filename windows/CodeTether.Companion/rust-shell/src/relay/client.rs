use super::error::Error;
use serde::{Serialize, de::DeserializeOwned};
use std::{io::Read, time::Duration};
use zeroize::Zeroizing;

pub(super) const ORIGIN: &str = "https://server.codetether.run/companion";
const RESPONSE_LIMIT: usize = 8_192;

pub(super) fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(10))
        .timeout(Duration::from_secs(20))
        .redirects(0)
        .build()
}

pub(super) fn decode<T: DeserializeOwned>(response: ureq::Response) -> Result<T, Error> {
    if response.status() != 200 {
        return Err(Error::from_status(response.status()));
    }
    let mut bytes = Zeroizing::new([0u8; RESPONSE_LIMIT + 1]);
    let mut reader = response.into_reader();
    let mut length = 0;
    loop {
        let count = reader
            .read(&mut bytes[length..])
            .map_err(|_| Error::InvalidResponse)?;
        if count == 0 {
            break;
        }
        length += count;
        if length > RESPONSE_LIMIT {
            return Err(Error::InvalidResponse);
        }
    }
    serde_json::from_slice(&bytes[..length]).map_err(|_| Error::InvalidResponse)
}

pub(super) fn send<T: Serialize>(
    request: ureq::Request,
    body: &T,
) -> Result<ureq::Response, Error> {
    let mut bytes = Zeroizing::new(Vec::with_capacity(256));
    serde_json::to_writer(&mut *bytes, body).map_err(|_| Error::InvalidInput)?;
    request
        .set("Content-Type", "application/json")
        .set("Cache-Control", "no-store")
        .send_bytes(&bytes)
        .map_err(Error::from_ureq)
}
