//! Non-secret authentication response types.
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub(super) struct Envelope<T> {
    pub data: T,
}
/// Non-secret authentication facts suitable for command output.
#[derive(Deserialize, Serialize)]
pub(crate) struct Facts {
    pub policies: Vec<String>,
    pub ttl: u64,
    pub renewable: bool,
}
