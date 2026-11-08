//! Encode only Monarch OAuth fields while preserving backend metadata.
use crate::oauth::Credentials;
use anyhow::{Result, anyhow};
use codetether_secrets::ProviderSecrets;
const FIELD: &str = "monarch_oauth";

pub(super) fn decode(record: Option<ProviderSecrets>) -> Result<Option<Credentials>> {
    let Some(value) = record.and_then(|mut record| record.extra.remove(FIELD)) else {
        return Ok(None);
    };
    let credentials: Credentials = serde_json::from_value(value)
        .map_err(|_| anyhow!("Invalid Monarch credential record in configured secret backend"))?;
    credentials.validate()?;
    Ok(Some(credentials))
}

pub(super) fn updated(
    mut record: ProviderSecrets,
    credentials: Option<&Credentials>,
) -> Result<ProviderSecrets> {
    match credentials {
        Some(credentials) => {
            credentials.validate()?;
            record.extra.insert(
                FIELD.into(),
                serde_json::to_value(credentials)
                    .map_err(|_| anyhow!("Could not encode Monarch credentials"))?,
            );
        }
        None => {
            record.extra.remove(FIELD);
        }
    }
    Ok(record)
}
