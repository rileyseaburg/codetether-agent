//! Cookie additions, removals, changes, and duplicate-key report.
use crate::browser::offline::cookie_parse::CookieRecord;
#[derive(Debug, serde::Serialize)]
/// Differences between two cookie jars, plus duplicate identity keys.
///
/// # Examples
/// ```
/// use codetether_browser::browser::offline::cookie_diff::CookieDelta;
/// let delta = CookieDelta { added: vec![], removed: vec![], changed: vec![], duplicates: vec![] };
/// assert!(delta.changed.is_empty());
/// ```
pub struct CookieDelta {
    pub added: Vec<CookieRecord>,
    pub removed: Vec<CookieRecord>,
    pub changed: Vec<Changed>,
    /// Cookie keys (name|domain|path) that appeared more than once in either input.
    pub duplicates: Vec<String>,
}
#[derive(Debug, serde::Serialize)]
/// Before and after records for one changed cookie identity.
///
/// # Examples
/// ```
/// use codetether_browser::browser::offline::{cookie_diff::Changed, cookie_parse::CookieRecord};
/// let before = CookieRecord { name: "a".into(), value: "1".into(), attrs: Default::default() };
/// let change = Changed { key: "a||".into(), before: before.clone(), after: before };
/// assert_eq!(change.before.name, change.after.name);
/// ```
pub struct Changed {
    pub key: String,
    pub before: CookieRecord,
    pub after: CookieRecord,
}
