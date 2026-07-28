//! Request security metadata used by route decisions and agents.

use super::resolve::{referrer_for, resolve_url};
use super::{Origin, SecurityPolicy};

/// Origin/referrer metadata for one page-initiated request.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{RequestSecurityMetadata, SecurityPolicy};
///
/// let metadata = RequestSecurityMetadata::new(
///     "https://app.test/a",
///     "/api",
///     &SecurityPolicy::default(),
/// );
/// assert!(metadata.same_origin);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestSecurityMetadata {
    /// Fully resolved request URL.
    pub resolved_url: String,
    /// Origin of the initiating page.
    pub request_origin: Origin,
    /// Origin of the target URL.
    pub target_origin: Origin,
    /// Deterministic referrer string, without URL fragment.
    pub referrer: Option<String>,
    /// Whether the request and page origins match.
    pub same_origin: bool,
    /// Whether the current policy allows the target origin.
    pub allowed_by_policy: bool,
}

impl RequestSecurityMetadata {
    /// Build metadata for a page URL and target URL.
    pub fn new(page_url: &str, target_url: &str, policy: &SecurityPolicy) -> Self {
        let resolved_url = resolve_url(page_url, target_url);
        let request_origin = Origin::parse(page_url);
        let target_origin = Origin::parse(&resolved_url);
        let same_origin = request_origin.is_same_origin(&target_origin);
        let allowed_by_policy = policy.allows(&request_origin, &target_origin);
        Self {
            resolved_url,
            request_origin,
            target_origin,
            referrer: referrer_for(page_url),
            same_origin,
            allowed_by_policy,
        }
    }
}
