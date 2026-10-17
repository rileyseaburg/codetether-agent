//! Load two cookie jars and serialize their comparison.
use super::diff;
use crate::browser::offline::cookie_parse::CookieRecord;
use anyhow::{Context, Result};
use std::path::Path;
/// Load two JSON cookie jars and return a formatted JSON difference report.
///
/// # Arguments
/// * `before_path` - Earlier JSON array of cookie records.
/// * `after_path` - Later JSON array of cookie records.
///
/// # Returns
/// The serialized cookie delta.
///
/// # Errors
/// Returns an error for unreadable files, malformed records, or serialization failures.
///
/// # Examples
/// ```no_run
/// let delta = codetether_browser::browser::offline::cookie_diff::run(
///     std::path::Path::new("before.json"), std::path::Path::new("after.json"))?;
/// assert!(delta.starts_with("{"));
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn run(before_path: &Path, after_path: &Path) -> Result<String> {
    let before = load(before_path).with_context(|| format!("read {}", before_path.display()))?;
    let after = load(after_path).with_context(|| format!("read {}", after_path.display()))?;
    Ok(serde_json::to_string_pretty(&diff(&before, &after))?)
}
fn load(path: &Path) -> Result<Vec<CookieRecord>> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}
