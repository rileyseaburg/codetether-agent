//! OpenAI-compatible client state.

use super::*;

/// Chat and embedding client for OpenAI-compatible APIs.
///
/// # Examples
/// ```
/// let client = codetether_provider_openai::OpenAIProvider::new("test-key".into()).unwrap();
/// assert!(format!("{client:?}").contains("OpenAIProvider"));
/// ```
pub struct OpenAIProvider {
    pub(super) client: Client<OpenAIConfig>,
    pub(super) provider_name: String,
    pub(super) api_key: Option<String>,
    pub(super) api_base: String,
    pub(super) http: HttpClient,
}
