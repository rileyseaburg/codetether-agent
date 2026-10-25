//! Read the standalone launcher's private renewal cache without modifying it.
use super::{
    cache_io,
    cookies::{Jar, now},
    rotation::Rotation,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    updated: u64,
    cookies: Vec<Rotation>,
}

pub(super) fn load(directory: &Path, base: &Jar) -> Option<(u64, Vec<Rotation>)> {
    let url = reqwest::Url::parse(super::ROTATE).ok()?;
    let header = base.header(&url, &[]);
    let key: String = Sha256::digest(header.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let bytes = cache_io::read(directory, &key)?;
    let mut record: Record = serde_json::from_slice(&bytes).ok()?;
    record.updated /= 1000;
    let instant = now();
    if record.updated == 0 || record.updated > instant || record.cookies.len() > 4 {
        return None;
    }
    for cookie in &mut record.cookies {
        cookie.expires /= 1000;
        if !cookie.valid() {
            return None;
        }
    }
    record
        .cookies
        .iter()
        .any(|cookie| {
            cookie.name == "__Secure-1PSIDTS"
                && !cookie.value.is_empty()
                && cookie.expires > instant
        })
        .then_some((record.updated, record.cookies))
}
