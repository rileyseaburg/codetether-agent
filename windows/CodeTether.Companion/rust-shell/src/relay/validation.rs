use super::error::Error;
use chrono::{DateTime, Duration, Utc};
use codetether_companion_protocol::PairReceipt;

pub(super) fn receipt(value: &PairReceipt) -> Result<DateTime<Utc>, Error> {
    request_id(&value.id)?;
    if value.device_token.len() != 43
        || !value
            .device_token
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        || !(15..=300).contains(&value.interval_seconds)
    {
        return Err(Error::InvalidResponse);
    }
    let expires = DateTime::parse_from_rfc3339(&value.expires_at)
        .map_err(|_| Error::InvalidResponse)?
        .with_timezone(&Utc);
    let now = Utc::now();
    if expires <= now || expires > now + Duration::minutes(61) {
        return Err(Error::InvalidResponse);
    }
    Ok(expires)
}

pub(super) fn request_id(value: &str) -> Result<(), Error> {
    let id = uuid::Uuid::parse_str(value).map_err(|_| Error::InvalidResponse)?;
    if id.hyphenated().to_string() != value {
        return Err(Error::InvalidResponse);
    }
    Ok(())
}
