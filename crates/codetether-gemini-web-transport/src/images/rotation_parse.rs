use super::Rotation;
use crate::images::cookies::now;
use std::collections::HashMap;

pub(in crate::images) fn from_header(header: &str) -> Option<Rotation> {
    let mut parts = header.split(';');
    let (name, value) = parts.next()?.split_once('=')?;
    let attrs: HashMap<_, _> = parts
        .map(|part| {
            let (key, value) = part.trim().split_once('=').unwrap_or((part.trim(), ""));
            (key.to_ascii_lowercase(), value)
        })
        .collect();
    if !attrs.contains_key("secure") || attrs.get("path") != Some(&"/") {
        return None;
    }
    let domain = attrs.get("domain")?.trim_start_matches('.');
    if !domain.eq_ignore_ascii_case("google.com") {
        return None;
    }
    let expires = if let Some(age) = attrs.get("max-age") {
        let age = age.parse::<i64>().ok()?;
        if age <= 0 {
            0
        } else {
            now().saturating_add(age as u64)
        }
    } else {
        crate::images::cookie_expiry::parse(attrs.get("expires")?)?
    };
    let cookie = Rotation {
        name: name.into(),
        value: value.into(),
        expires,
    };
    cookie.valid().then_some(cookie)
}
