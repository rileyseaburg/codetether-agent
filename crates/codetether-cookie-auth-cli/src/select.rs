//! Cookie import select responsibilities.
use crate::{names::preferred_cookie_names, row::CookieRow};
use std::collections::HashMap;

pub(super) fn select_cookie_rows(
    rows: &[CookieRow],
    provider_id: &str,
    keep_all: bool,
) -> (Vec<CookieRow>, usize, usize) {
    let now_epoch = chrono::Utc::now().timestamp();
    let allowed = preferred_cookie_names(provider_id);
    let mut selected_by_name: HashMap<String, CookieRow> = HashMap::new();
    let mut dropped_expired = 0usize;
    let mut dropped_non_auth = 0usize;

    for row in rows {
        if row.name.is_empty() {
            continue;
        }
        if row.expires_epoch > 0 && row.expires_epoch <= now_epoch {
            dropped_expired += 1;
            continue;
        }
        if !keep_all && !allowed.is_empty() && !allowed.iter().any(|name| *name == row.name) {
            dropped_non_auth += 1;
            continue;
        }
        match selected_by_name.get(&row.name) {
            Some(existing) if existing.expires_epoch >= row.expires_epoch => {}
            _ => {
                selected_by_name.insert(row.name.clone(), row.clone());
            }
        }
    }

    let mut selected: Vec<CookieRow> = selected_by_name.into_values().collect();
    selected.sort_by(|left, right| left.name.cmp(&right.name));
    (selected, dropped_expired, dropped_non_auth)
}
