//! Trace redirects and cookies without launching a browser.
//!
//! [run] produces JSON from a bounded traversal; [AuthTrace] and [Step]
//! describe the resulting hop history and final cookie jar.
use anyhow::Result;
#[cfg(feature = "tetherscript")]
mod cookie;
mod model;
#[cfg(feature = "tetherscript")]
mod request;
mod url;
mod walk;
pub use model::{AuthTrace, Step};
/// Trace an HTTP GET redirect chain while carrying cookies across hops.
///
/// # Arguments
/// * `url` - Initial URL to request.
/// * `max_redirects` - Maximum redirects followed after the initial response.
///
/// # Returns
/// Pretty-printed JSON containing steps, final URL, truncation, and cookies.
///
/// # Errors
/// Returns an error if HTTP setup or a request fails, serialization fails, or
/// the TetherScript feature is disabled.
///
/// # Examples
/// ```no_run
/// let report = codetether_browser::browser::offline::auth_trace::run("https://example.test/", 5)?;
/// assert!(report.contains("redirect_count"));
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn run(url: &str, max_redirects: u8) -> Result<String> {
    let trace = walk::walk(url, max_redirects)?;
    Ok(serde_json::to_string_pretty(&trace)?)
}

#[cfg(all(test, feature = "tetherscript"))]
mod tests;
