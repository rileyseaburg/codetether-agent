use super::{cache::Cache, cookies::Jar, netscape, rotation::Rotation};
use anyhow::{Result, anyhow};
use std::{path::PathBuf, time::Duration};
use tokio::sync::Mutex;

/// Native authenticated image client with serialized cookie renewal.
///
/// Cookies and signed URLs are deliberately excluded from `Debug` output.
///
/// # Examples
/// ```no_run
/// use codetether_gemini_web_transport::images::ImageClient;
/// let export = std::fs::read_to_string("cookies.txt")?;
/// let client = ImageClient::new(&export, "private-cache".into())?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct ImageClient {
    pub(super) http: reqwest::Client,
    pub(super) cache: Cache,
    pub(super) state: Mutex<State>,
}
pub(super) struct State {
    pub base: Jar,
    pub updates: Vec<Rotation>,
    pub rotated_at: u64,
}
impl ImageClient {
    /// Parse a Netscape export and configure a private renewal cache.
    ///
    /// # Arguments
    /// * `netscape_cookies` - Export contents, never a filename or raw header.
    /// * `cache_dir` - Directory exclusively owned by this user (0700 on Unix).
    ///
    /// # Returns
    /// A client; construction makes no network requests and does not change the export.
    ///
    /// # Errors
    /// Returns a credential-free error for invalid cookies or HTTP configuration.
    /// Cache permissions are checked before reading or writing during generation.
    ///
    /// # Examples
    /// See the [`ImageClient`] example.
    pub fn new(netscape_cookies: &str, cache_dir: PathBuf) -> Result<Self> {
        let base = netscape::parse(netscape_cookies)?;
        let http = reqwest::Client::builder()
            // Google's image bootstrap rejects this client's HTTP/2 request.
            // Select HTTP/1.1 upfront; never retry an image-generation POST.
            .http1_only()
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(Duration::from_secs(20))
            .timeout(Duration::from_secs(180))
            .user_agent(super::USER_AGENT)
            .build()
            .map_err(|_| anyhow!("image_http_configuration_failed"))?;
        Ok(Self {
            http,
            cache: Cache::new(cache_dir, netscape_cookies),
            state: Mutex::new(State {
                base,
                updates: Vec::new(),
                rotated_at: 0,
            }),
        })
    }
}
