//! Diff two JSON cookie jars. Keyed on (name, domain, path) via cookie_diff_index.
//! Any record-level difference counts as `changed`, not just value.

use anyhow::{Context, Result};
use std::path::Path;

use super::cookie_diff_index::index;
use super::cookie_parse::CookieRecord;

#[derive(Debug, serde::Serialize)]
pub struct CookieDelta {
    pub added: Vec<CookieRecord>,
    pub removed: Vec<CookieRecord>,
    pub changed: Vec<Changed>,
    /// Cookie keys (name|domain|path) that appeared more than once in either input.
    pub duplicates: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct Changed {
    pub key: String,
    pub before: CookieRecord,
    pub after: CookieRecord,
}

pub fn run(before_path: &Path, after_path: &Path) -> Result<String> {
    let before = load(before_path).with_context(|| format!("read {}", before_path.display()))?;
    let after = load(after_path).with_context(|| format!("read {}", after_path.display()))?;
    Ok(serde_json::to_string_pretty(&diff(&before, &after))?)
}

fn load(path: &Path) -> Result<Vec<CookieRecord>> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn diff(before: &[CookieRecord], after: &[CookieRecord]) -> CookieDelta {
    let (bmap, mut duplicates) = index(before);
    let (amap, adup) = index(after);
    duplicates.extend(adup);
    duplicates.sort();
    duplicates.dedup();
    let added = amap
        .iter()
        .filter(|(k, _)| !bmap.contains_key(*k))
        .map(|(_, c)| (*c).clone())
        .collect();
    let removed = bmap
        .iter()
        .filter(|(k, _)| !amap.contains_key(*k))
        .map(|(_, c)| (*c).clone())
        .collect();
    let changed = bmap
        .iter()
        .filter_map(|(k, b)| {
            amap.get(k).and_then(|a| {
                (a != b).then(|| Changed {
                    key: k.clone(),
                    before: (*b).clone(),
                    after: (*a).clone(),
                })
            })
        })
        .collect();
    CookieDelta {
        added,
        removed,
        changed,
        duplicates,
    }
}
