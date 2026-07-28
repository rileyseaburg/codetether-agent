//! Cookie first-pair and default construction.

use super::model::{Cookie, SameSite};
use super::path::default_cookie_path;

pub(crate) fn first_pair(part: Option<&str>) -> Result<(&str, &str), String> {
    let Some(first) = part.filter(|item| !item.is_empty()) else {
        return Err("cookie is empty".into());
    };
    let Some((name, value)) = first.split_once('=') else {
        return Err("cookie must start with name=value".into());
    };
    if name.trim().is_empty() {
        return Err("cookie name is empty".into());
    }
    Ok((name.trim(), value.trim()))
}

pub(crate) fn new_cookie(name: &str, value: &str, host: &str, url: &str) -> Cookie {
    Cookie {
        name: name.into(),
        value: value.into(),
        domain: host.into(),
        path: default_cookie_path(url),
        secure: false,
        http_only: false,
        same_site: SameSite::default(),
        expires_at: None,
        host_only: true,
    }
}
