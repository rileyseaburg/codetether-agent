//! BrowserPage security metadata APIs.

use super::{Origin, RequestSecurityMetadata, SecurityPolicy};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return the page's current normalized origin.
    pub fn current_origin(&self) -> Origin {
        Origin::parse(&self.session.url)
    }

    /// Return the page security policy.
    pub fn security_policy(&self) -> &SecurityPolicy {
        &self.security_policy
    }

    /// Replace the page security policy.
    pub fn set_security_policy(&mut self, policy: SecurityPolicy) {
        self.security_policy = policy;
    }

    /// Allow a cross-origin target for this page.
    pub fn allow_origin(&mut self, url: impl AsRef<str>) -> Origin {
        let origin = Origin::parse(url.as_ref());
        self.security_policy.allow_origin(origin.clone());
        origin
    }

    /// Clear page-specific cross-origin allowances.
    pub fn clear_allowed_origins(&mut self) {
        self.security_policy.clear_allowed_origins();
    }

    /// Return request origin/referrer metadata for a target URL.
    pub fn request_security_metadata(&self, url: impl AsRef<str>) -> RequestSecurityMetadata {
        RequestSecurityMetadata::new(&self.session.url, url.as_ref(), &self.security_policy)
    }

    /// Return whether a request target is same-origin or explicitly allowed.
    pub fn is_request_allowed(&self, url: impl AsRef<str>) -> bool {
        self.request_security_metadata(url).allowed_by_policy
    }
}
