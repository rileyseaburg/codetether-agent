use crate::Error;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

fn random<const N: usize>() -> Result<[u8; N], Error> {
    let mut bytes = [0; N];
    getrandom::fill(&mut bytes).map_err(|_| Error::Entropy)?;
    Ok(bytes)
}
pub(crate) fn id() -> Result<String, Error> {
    let mut bytes = random::<16>()?;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(uuid::Uuid::from_bytes(bytes).to_string())
}
pub(crate) fn code() -> Result<String, Error> {
    Ok(random::<6>()?
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect())
}
pub(crate) fn token() -> Result<String, Error> {
    Ok(URL_SAFE_NO_PAD.encode(random::<32>()?))
}
pub(crate) fn timestamp(ms: i64) -> Result<String, Error> {
    chrono::DateTime::from_timestamp_millis(ms)
        .map(|time| time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        .ok_or(Error::Configuration)
}
