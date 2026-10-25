use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub(super) struct Cookie {
    pub domain: String,
    pub subdomains: bool,
    pub path: String,
    pub secure: bool,
    pub expires: u64,
    pub name: String,
    pub value: String,
}
pub(super) struct Jar(pub Vec<Cookie>);
pub(super) fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub(super) fn valid_value(value: &str) -> bool {
    value
        .bytes()
        .all(|c| (0x21..=0x7e).contains(&c) && !b"\";,\\".contains(&c))
}
pub(super) fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&c))
}
