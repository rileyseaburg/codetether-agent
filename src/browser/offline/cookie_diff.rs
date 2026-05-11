//! Diff two JSON cookie jars (lists of CookieRecord) and emit added/removed/changed.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

use super::cookie_parse::CookieRecord;

#[derive(Debug, serde::Serialize)]
pub struct CookieDelta {
    pub added: Vec<CookieRecord>,
    pub removed: Vec<CookieRecord>,
    pub changed: Vec<Changed>,
}

#[derive(Debug, serde::Serialize)]
pub struct Changed {
    pub name: String,
    pub before: String,
    pub after: String,
}

pub fn run(before_path: &Path, after_path: &Path) -> Result<String> {
    let before = load(before_path).with_context(|| format!("read {}", before_path.display()))?;
    let after = load(after_path).with_context(|| format!("read {}", after_path.display()))?;
    let delta = diff(&before, &after);
    Ok(serde_json::to_string_pretty(&delta)?)
}

fn load(path: &Path) -> Result<Vec<CookieRecord>> {
    let text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn diff(before: &[CookieRecord], after: &[CookieRecord]) -> CookieDelta {
    let bmap: HashMap<&str, &CookieRecord> = before.iter().map(|c| (c.name.as_str(), c)).collect();
    let amap: HashMap<&str, &CookieRecord> = after.iter().map(|c| (c.name.as_str(), c)).collect();
    let added = after.iter().filter(|c| !bmap.contains_key(c.name.as_str())).cloned().collect();
    let removed = before.iter().filter(|c| !amap.contains_key(c.name.as_str())).cloned().collect();
    let changed = before
        .iter()
        .filter_map(|b| amap.get(b.name.as_str()).and_then(|a| (a.value != b.value).then(|| Changed {
            name: b.name.clone(),
            before: b.value.clone(),
            after: a.value.clone(),
        })))
        .collect();
    CookieDelta { added, removed, changed }
}
