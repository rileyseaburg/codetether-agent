//! State for the existing application protocol conventions.
use reqwest::Client;
/// Z.AI provider retaining the existing application's protocol conventions.
///
/// # Examples
///
///     use codetether_provider_zai::application::ZaiProvider;
///     let provider = ZaiProvider::with_base_url("example-secret".into(), "https://example.test/v4".into()).unwrap();
///     assert!(!format!("{provider:?}").contains("example-secret"));
pub struct ZaiProvider {
    /// Shared provider HTTP pool.
    pub(super) client: Client,
    /// Credential, redacted by the debug implementation.
    pub(super) api_key: String,
    /// Unmodified endpoint prefix supplied by application configuration.
    pub(super) base_url: String,
}
