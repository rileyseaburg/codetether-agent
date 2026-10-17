//! Serde defaults for PRD and story contracts.

pub(crate) fn default_http_method() -> String {
    "GET".to_string()
}

pub(crate) fn default_http_status() -> u16 {
    200
}

pub(crate) fn default_http_timeout_secs() -> u64 {
    30
}

pub(crate) fn default_priority() -> u8 {
    1
}

pub(crate) fn default_complexity() -> u8 {
    3
}

pub(crate) fn default_version() -> String {
    "1.0".to_string()
}
