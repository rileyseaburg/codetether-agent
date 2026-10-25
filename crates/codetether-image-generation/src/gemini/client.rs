//! Construct the native client, optionally migrating the local launcher's cache.
use anyhow::Result;
use codetether_gemini_web_transport::images::ImageClient;
use std::path::{Path, PathBuf};

pub(super) fn create(cookies: &str, data: &Path) -> Result<ImageClient> {
    let client = ImageClient::new(cookies, data.join("gemini_web_cookie_cache"))?;
    if std::env::var("CODETETHER_DISABLE_ENV_FALLBACK").as_deref() == Ok("1")
        || std::env::var_os("GEMINI_WEB_COOKIE_FILE").is_none()
    {
        return Ok(client);
    }
    let cache = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")));
    Ok(match cache {
        Some(root) => client.with_node_cache(&root.join("codetether/gemini-web")),
        None => client,
    })
}
