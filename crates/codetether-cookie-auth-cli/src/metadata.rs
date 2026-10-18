//! Build cookie import metadata while preserving cookie selection and expiry.
use crate::{expiry::cookie_expiry_bounds, render::render_netscape_cookie_file, row::CookieRow};
use serde_json::{Value, json};
use std::collections::HashMap;

pub(super) fn build(
    selected: &[CookieRow],
    keep_all: bool,
    dropped_expired: usize,
    dropped_non_auth: usize,
) -> (HashMap<String, Value>, Option<i64>) {
    let rendered = render_netscape_cookie_file(selected);
    let now = chrono::Utc::now();
    let (earliest_expiry, latest_expiry) = cookie_expiry_bounds(selected);
    let cookie_names: Vec<String> = selected.iter().map(|row| row.name.clone()).collect();
    let mut extra = HashMap::new();
    extra.insert("cookies".to_string(), json!(rendered));
    extra.insert("cookie_format".to_string(), json!("netscape"));
    extra.insert("imported_at".to_string(), json!(now.to_rfc3339()));
    extra.insert("cookie_count".to_string(), json!(selected.len()));
    extra.insert("cookie_names".to_string(), json!(cookie_names));
    extra.insert("dropped_expired".to_string(), json!(dropped_expired));
    extra.insert("dropped_non_auth".to_string(), json!(dropped_non_auth));
    extra.insert("keep_all".to_string(), json!(keep_all));
    extra.insert(
        "strategy".to_string(),
        json!(if keep_all {
            "cookies_all_v1"
        } else {
            "cookies_auth_subset_v1"
        }),
    );

    super::metadata_expiry::insert(&mut extra, earliest_expiry, latest_expiry);
    (extra, earliest_expiry)
}
