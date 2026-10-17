//! Relative prefix resolution and segment-aware matching.

use anyhow::{Result, bail};

pub(super) fn resolve(current_path: &str, prefix: Option<&str>) -> Result<Option<String>> {
    let Some(prefix) = prefix else {
        return Ok(None);
    };
    if prefix.is_empty() || prefix.ends_with('/') || prefix.contains("//") {
        bail!("agent path prefix must be non-empty and must not end with `/`");
    }
    let resolved = if prefix.starts_with('/') {
        prefix.to_string()
    } else {
        format!("{current_path}/{prefix}")
    };
    if resolved != "/root" && !resolved.starts_with("/root/") {
        bail!("absolute agent paths must start with `/root`");
    }
    Ok(Some(resolved))
}

pub(super) fn matches(path: &str, prefix: Option<&str>) -> bool {
    prefix.is_none_or(|prefix| {
        prefix == "/root"
            || path == prefix
            || path
                .strip_prefix(prefix)
                .is_some_and(|suffix| suffix.starts_with('/'))
    })
}

#[cfg(test)]
#[path = "prefix_tests.rs"]
mod tests;
