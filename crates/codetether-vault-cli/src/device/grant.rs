//! Device authorization response and default polling interval.
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Grant {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    #[serde(default = "interval")]
    pub interval: u64,
}

fn interval() -> u64 {
    5
}
