//! Compare cookie records by identity and preserve last-entry indexing.
use super::{Changed, CookieDelta, index::index};
use crate::browser::offline::cookie_parse::CookieRecord;
/// Compare complete cookie records using their name, domain, and path.
///
/// # Arguments
/// * `before` - Earlier cookie jar.
/// * `after` - Later cookie jar.
///
/// # Returns
/// Added, removed, and changed records plus sorted duplicate keys. The final record
/// wins when a jar repeats a key; record vector order follows hash-map iteration.
///
/// # Examples
/// ```
/// use codetether_browser::browser::offline::cookie_diff::diff;
/// let delta = diff(&[], &[]);
/// assert!(delta.added.is_empty());
/// ```
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
