use super::{cache_io, rotation::Rotation};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub(super) struct Cache {
    pub directory: PathBuf,
    pub key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Record {
    pub source: String,
    pub updated: u64,
    pub cookies: Vec<Rotation>,
}
impl Cache {
    pub fn new(directory: PathBuf, source: &str) -> Self {
        Self {
            directory,
            key: Sha256::digest(source.as_bytes())
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
        }
    }
    pub fn load(&self) -> Option<Record> {
        let bytes = cache_io::read(&self.directory, &self.key)?;
        let record: Record = serde_json::from_slice(&bytes).ok()?;
        (record.source == self.key
            && record.cookies.len() <= 4
            && record
                .cookies
                .iter()
                .any(|c| c.name == "__Secure-1PSIDTS" && !c.value.is_empty())
            && record.cookies.iter().all(Rotation::valid))
        .then_some(record)
    }
    pub fn save(&self, updated: u64, cookies: &[Rotation]) -> Result<()> {
        let record = Record {
            source: self.key.clone(),
            updated,
            cookies: cookies.to_vec(),
        };
        let bytes = serde_json::to_vec(&record)
            .map_err(|_| anyhow::anyhow!("cookie_cache_encode_failed"))?;
        cache_io::write(&self.directory, &self.key, &bytes)
    }
}
