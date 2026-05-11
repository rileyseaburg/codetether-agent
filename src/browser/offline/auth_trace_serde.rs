//! Serialize a tetherscript Cookie into the public CookieRecord JSON shape.

#![cfg(feature = "tetherscript")]

use std::collections::BTreeMap;

use super::cookie_parse::CookieRecord;

pub(crate) fn serialize_cookie(c: &tetherscript::browser_session::Cookie) -> CookieRecord {
    let mut attrs = BTreeMap::new();
    attrs.insert("domain".into(), c.domain.clone());
    attrs.insert("path".into(), c.path.clone());
    if c.secure {
        attrs.insert("secure".into(), String::new());
    }
    if c.http_only {
        attrs.insert("httponly".into(), String::new());
    }
    if let Some(exp) = c.expires_at {
        attrs.insert("expires_at".into(), exp.to_string());
    }
    CookieRecord { name: c.name.clone(), value: c.value.clone(), attrs }
}
